use pdb::FallibleIterator;
use regex::Regex;
use serde::Serialize;
use std::error::Error;

#[allow(dead_code)]
#[derive(Serialize)]
pub struct Offset {
    pub symbol: String,
    pub offset: u64,
}

pub fn get_member_offsets(
    pdb_path: &str,
    symbol_name: &str,
    member_name: &str,
) -> Result<Vec<Offset>, Box<dyn Error>> {
    let file = std::fs::File::open(pdb_path)?;
    let mut pdb = pdb::PDB::open(file)?;
    let mut offsets: Vec<Offset> = Vec::new();

    let type_information = pdb.type_information()?;
    let mut type_finder = type_information.finder();
    let mut type_iter = type_information.iter();
    let symbol_name_regex = Regex::new(symbol_name)?;
    let member_name_regex = Regex::new(member_name)?;

    while let Some(typ) = type_iter.next()? {
        type_finder.update(&type_iter);

        if let Ok(pdb::TypeData::Class(class)) = typ.parse() {
            let name = class.name.to_string().into_owned();

            if symbol_name_regex.find(&name).is_some() {
                if let Some(fields) = class.fields {
                    match type_finder.find(fields)?.parse()? {
                        pdb::TypeData::FieldList(data) => {
                            for field in &data.fields {
                                match *field {
                                    pdb::TypeData::Member(ref data) => {
                                        if member_name_regex.find(&data.name.to_string()).is_some()
                                        {
                                            offsets.push(Offset {
                                                symbol: format!("{}.{}", class.name, data.name),
                                                offset: data.offset,
                                            });
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(offsets)
}

pub fn get_struct_offsets(
    pdb_path: &str,
    symbol_name: &str,
) -> Result<Vec<Vec<Offset>>, Box<dyn Error>> {
    let file = std::fs::File::open(pdb_path)?;
    let mut pdb = pdb::PDB::open(file)?;
    let mut offsets: Vec<Vec<Offset>> = Vec::new();

    let type_information = pdb.type_information()?;
    let mut type_finder = type_information.finder();
    let mut type_iter = type_information.iter();
    let symbol_name_regex = Regex::new(symbol_name)?;

    while let Some(typ) = type_iter.next()? {
        type_finder.update(&type_iter);

        if let Ok(pdb::TypeData::Class(class)) = typ.parse() {
            let name = class.name.to_string().into_owned();

            if symbol_name_regex.find(&name).is_some() {
                let mut struct_offsets: Vec<Offset> = Vec::new();
                if let Some(fields) = class.fields {
                    match type_finder.find(fields)?.parse()? {
                        pdb::TypeData::FieldList(data) => {
                            for field in &data.fields {
                                match *field {
                                    pdb::TypeData::Member(ref data) => {
                                        struct_offsets.push(Offset {
                                            symbol: format!("{}.{}", class.name, data.name),
                                            offset: data.offset,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                            if struct_offsets.len() > 0 {
                                offsets.push(struct_offsets);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(offsets)
}

pub fn get_data_offsets(pdb_path: &str, symbol_name: &str) -> Result<Vec<Offset>, Box<dyn Error>> {
    let file = std::fs::File::open(pdb_path)?;
    let mut pdb = pdb::PDB::open(file)?;
    let symbol_table = pdb.global_symbols()?;
    let mut symbols = symbol_table.iter();
    let mut offsets: Vec<Offset> = Vec::new();
    let symbol_name_regex = Regex::new(symbol_name)?;

    while let Some(symbol) = symbols.next()? {
        match symbol.parse()? {
            pdb::SymbolData::Data(data) => {
                if symbol_name_regex.find(&data.name.to_string()).is_some() {
                    offsets.push(Offset {
                        symbol: data.name.to_string().into_owned(),
                        offset: data.offset.offset as u64,
                    });
                }
            }
            _ => {}
        }
    }
    Ok(offsets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pdb() {
        if let Ok(offsets) = get_member_offsets(
            "tests/foo.pdb",
            "output_adapter_data<wchar_t.*::string_output_adapter<wchar_t>.*",
            ".*output_adapter",
        ) {
            assert_eq!(offsets.len(), 1);
        }
        if let Ok(offsets) = get_data_offsets("tests/foo.pdb", "__dcr.*_wide_environment") {
            assert_eq!(offsets.len(), 1);
        }
    }
}
