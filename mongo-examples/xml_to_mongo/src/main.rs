// Constructors externos (Se declaran en Cargo.toml)
extern crate xmlJSON;
extern crate rustc_serialize;
extern crate walkdir;
// Librerias externas (Necesitan importacion en Cargo.toml o un constructor externo)
use rustc_serialize::json::ToJson;
use xmlJSON::XmlDocument;
use rustc_serialize::json;
use walkdir::WalkDir;
use bson;
use serde_json::{Map, Value};
use mongodb::Client;
use tokio;
// Librerias del sistema o nativas
use std::str::FromStr;
use std::io::{Read};
use std::ffi::OsStr;
use std::path::Path;
use std::fs;
use std::env;
use std::str;
use std::io;

#[tokio::main]  // Necesario para usar las funciones asincronas de Mongo DB
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Recibir parametros del sistema
    let args: Vec<String> = env::args().collect(); // Acceder a la coleccion de args
    let new_env = &args[1];
    let env : String = new_env.to_string(); // Convertir args &str a String
    let client_url : &str;

    // Selector de entornos para conectar a Mongo DB
    if env == "dev" || env == "qa" {
        client_url = "mongodb://localhost:27017";
    }
    else if env == "test" || env == "prod" {
        client_url = "mongodb://mongo-0.mongo:27017,mongo-1.mongo:27017,mongo-2.mongo:27017/admin?replicaSet=rs0";
    }
    else {
        client_url = "mongodb://mongo-0.mongo:27017,mongo-1.mongo:27017,mongo-2.mongo:27017/admin?replicaSet=rs0";
    }

    // Conexion al cliente Mongo BD - Base de datos - Colección
    let _client = Client::with_uri_str(client_url).await?;    
    let _db = _client.database("test");
    let _collection = _db.collection("clinicaltrials");

    // Ruta temporal
    let mut _tmpath : &str = "./tmp/";

    // Lectura del path de almacenamiento
    for entry in WalkDir::new("./compress").into_iter().filter_map(|e| e.ok()) {
        // Filtro para separar archios de folders 
        if entry.metadata().unwrap().is_file() {
            // Filtro para archivos ZIP
            if entry.path().extension() == Some(OsStr::new("zip")) {
                // Lectura de archivos zip
                let file = match fs::File::open(entry.path()) {
                    Ok(file) => file,
                    Err(why) => panic!(
                        "ERROR: Could not open file, {}: {}", entry.path().display(), why.to_string()
                    ),
                };
                // UTF-8 del zip
                let mut archive = zip::ZipArchive::new(file).unwrap();
                // Procesado de cada ruta interna del archivo zip
                for i in 0..archive.len() {
                    // Seleccionar index
                    let mut _file = archive.by_index(i).unwrap();
                    // Path interno
                    let _outpath = match _file.enclosed_name() {
                        Some(path) => path.to_owned(),
                        None => continue,
                    };
                    // Reconstruccion del path
                    let _tmp = format!("{}{}", _tmpath, _outpath.display());
                    // Filtro de rchivos y folders internos del zip
                    if (&*_file.name()).ends_with('/') {
                        fs::create_dir_all(&_tmp).unwrap();
                    } else {
                        // Craecion de ruta temporal si no existe
                        if let Some(p) = _outpath.parent() {
                            if !p.exists() {
                                fs::create_dir_all(&_tmp).unwrap();
                            }
                        }
                        // Apuntador temporal
                        let mut outfile = fs::File::create(&_tmp).unwrap();
                        // Copiado temporal
                        io::copy(&mut _file, &mut outfile).unwrap();
                    }
                }
            }
        }
    }
    // Lectura de archivos temporales y rutas
    for e in WalkDir::new(_tmpath).into_iter().filter_map(|e| e.ok()) {
        // Filtro de archivos y folders
        if e.metadata().unwrap().is_file() {
            // Busqueda de archivos XML
            if e.path().extension() == Some(OsStr::new("xml")) {
                // Lectura del XML
                let mut f = fs::File::open(e.path()).expect("file not found");
                let mut contents = String::new();
                // Conversion a String
                f.read_to_string(&mut contents).expect("something went wrong reading the file");
                // Bytes Array String a la estructura de XML
                let document: XmlDocument = XmlDocument::from_str(&mut contents).unwrap();
                // Parse de XML a JSON
                let jsn: json::Json = document.to_json(); 
                // Mapeo de la estructura en JSON para insertar a Mongo
                let _value : Map<String, Value> = serde_json::from_str(&jsn.to_string())?;
                // Conversion de JSON a Document (Propio de BSON)
                let _document = bson::to_document(&_value)?;
                // Insert a Mongo DB
                _collection.insert_one(_document, None).await?;
            }
        }
    }
    // Llamado de funcion para eliminar archivos temporales y residucos de la ejecicion
    remove_dir_contents(_tmpath).unwrap();
    // Eliminacion del path tmp
    fs::remove_dir(_tmpath)?;
    // Resultado de la funcion
    Ok(()) // <- OK significa que todo resulto bien
}

// Funcion de limpueza
fn remove_dir_contents<P: AsRef<Path>>(path: P) -> io::Result<()> {
    // Ciclo para limpier todas las rutas
    for entry in fs::read_dir(path)? {
        // Extraccion del path a limpiar
        let entry = entry?;
        let path = entry.path();
        // FIltro para saber si es un folder
        if entry.file_type()?.is_dir() {
            // Recursividad en la funcion para ingresar al folder
            remove_dir_contents(&path)?;
            // Eliminacion del folder detectado
            fs::remove_dir(path)?;
        } else {
            // Eliminacion del archivo detectado
            fs::remove_file(path)?;
        }
    }
    // Reusltado
    Ok(())
}