//! Activar y desactivar una fuente de usuario de prueba, sin tocar las de
//! verdad: carpetas temporales y una clave de registro propia en HKCU que se
//! borra al acabar. No se carga nada en Windows (tocar_windows = false).

use std::{collections::HashSet, path::Path};
use tunafonts_lib::fuentes::{self, cache::Cache, cambios, leer, registro, Entorno, Origen};
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

struct Limpieza(String);
impl Drop for Limpieza {
    fn drop(&mut self) {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let _ = hkcu.delete_subkey_all(&self.0);
        // Y la carpeta de pruebas, si se ha quedado vacía.
        let _ = hkcu.delete_subkey(r"Software\tuna-fonts-pruebas");
    }
}

#[test]
fn instalar_desactivar_y_activar_una_de_usuario() {
    let tmp = tempfile::tempdir().unwrap();
    let base = format!(r"Software\tuna-fonts-pruebas\{}", std::process::id());
    let _limpia = Limpieza(base.clone());
    let ent = Entorno {
        fuentes_sistema: tmp.path().join("sistema"),
        fuentes_usuario: tmp.path().join("usuario"),
        clave_sistema: format!(r"{base}\Sistema"),
        clave_usuario: format!(r"{base}\Fonts"),
        datos: tmp.path().join("datos"),
        tocar_windows: false,
        de_windows: |_| true,
        de_programas: HashSet::new(),
    };
    let fuente = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/GochiHand-Regular.ttf");

    // Leer: familia y categoría.
    let caras = leer::caras(&fuente).unwrap();
    assert_eq!(caras[0].familia, "Gochi Hand");
    assert_eq!(caras[0].categoria, "script");
    assert!(!leer::caracteres(&fuente, 500).unwrap().is_empty());

    // Instalar para el usuario.
    let instalada = cambios::instalar_usuario(&ent, &fuente, &caras).unwrap();
    assert!(instalada.is_file());
    let reg = registro::leer(registro::hkcu(), &ent.clave_usuario);
    assert_eq!(reg.len(), 1);
    assert_eq!(reg[0].0, "Gochi Hand (TrueType)");
    assert_eq!(Path::new(&reg[0].1), instalada);

    let mut cache = Cache::default();
    let entradas = fuentes::entradas(&ent, &[]);
    cache.actualizar(&entradas);
    let f = cache.familias.iter().find(|f| f.nombre == "Gochi Hand").unwrap();
    assert!(f.activa);
    assert_eq!(f.origen, Origen::Biblioteca);

    // Desactivar: fuera del registro y a la carpeta de la app, sin borrar.
    let mut apagadas = HashSet::new();
    let r = cambios::alternar(&ent, &entradas, &cache, &["Gochi Hand".into()], false, false, &mut apagadas);
    assert_eq!(r.cambiadas, 1, "{:?}", r.errores);
    assert!(!instalada.exists());
    assert!(registro::leer(registro::hkcu(), &ent.clave_usuario).is_empty());
    let g = fuentes::guardadas(&ent);
    assert_eq!(g.len(), 1);
    assert!(g[0].archivo.starts_with(ent.desactivadas()) && g[0].archivo.is_file());

    let entradas = fuentes::entradas(&ent, &[]);
    cache.actualizar(&entradas);
    let f = cache.familias.iter().find(|f| f.nombre == "Gochi Hand").unwrap();
    assert!(!f.activa);

    // Desactivar otra vez no hace nada.
    let r = cambios::alternar(&ent, &entradas, &cache, &["Gochi Hand".into()], false, false, &mut apagadas);
    assert_eq!(r.cambiadas, 0);

    // Activar: vuelve a su sitio y a su entrada del registro.
    let r = cambios::alternar(&ent, &entradas, &cache, &["Gochi Hand".into()], true, false, &mut apagadas);
    assert_eq!(r.cambiadas, 1, "{:?}", r.errores);
    assert!(instalada.is_file());
    let reg = registro::leer(registro::hkcu(), &ent.clave_usuario);
    assert_eq!(reg, vec![("Gochi Hand (TrueType)".to_string(), instalada.to_string_lossy().into_owned())]);
    assert!(fuentes::guardadas(&ent).is_empty());

    let entradas = fuentes::entradas(&ent, &[]);
    cache.actualizar(&entradas);
    assert!(cache.familias.iter().find(|f| f.nombre == "Gochi Hand").unwrap().activa);
}

#[test]
fn las_imprescindibles_no_se_tocan() {
    for f in ["Segoe UI", "Segoe UI Variable", "Segoe UI Emoji", "Segoe Fluent Icons", "Segoe MDL2 Assets", "Marlett", "Tahoma", "Microsoft Sans Serif", "Arial", "Consolas"] {
        assert!(fuentes::es_imprescindible(f), "{f}");
    }
    for f in ["Arial Narrow", "Georgia", "Times New Roman", "Segoe Print"] {
        assert!(!fuentes::es_imprescindible(f), "{f}");
    }
}

/// En la carpeta de fuentes del sistema, solo las que trae Windows van con
/// candado; las instaladas a mano «para todos» (u Office) salen como de usuario.
#[test]
fn las_instaladas_para_todos_no_cuentan_como_del_sistema() {
    let tmp = tempfile::tempdir().unwrap();
    let ent = Entorno {
        fuentes_sistema: tmp.path().join("sistema"),
        fuentes_usuario: tmp.path().join("usuario"),
        clave_sistema: r"Software\tuna-fonts-pruebas\no-existe".into(),
        clave_usuario: r"Software\tuna-fonts-pruebas\no-existe".into(),
        datos: tmp.path().join("datos"),
        tocar_windows: false,
        // Simula el propietario: solo «de-windows.ttf» es de TrustedInstaller.
        de_windows: |p| p.file_name().is_some_and(|n| n == "de-windows.ttf"),
        // Y «de-office.ttf» la trae Office.
        de_programas: ["de-office.ttf".to_string()].into(),
    };
    let fuente = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/GochiHand-Regular.ttf");
    std::fs::create_dir_all(&ent.fuentes_sistema).unwrap();
    std::fs::copy(&fuente, ent.fuentes_sistema.join("de-windows.ttf")).unwrap();
    std::fs::copy(&fuente, ent.fuentes_sistema.join("a-mano.ttf")).unwrap();
    std::fs::copy(&fuente, ent.fuentes_sistema.join("DE-OFFICE.TTF")).unwrap();

    let origen = |nombre: &str| {
        fuentes::entradas(&ent, &[]).into_iter().find(|e| e.ruta.file_name().unwrap() == nombre).unwrap().origen
    };
    assert_eq!(origen("de-windows.ttf"), Origen::Sistema);
    assert_eq!(origen("a-mano.ttf"), Origen::Usuario);
    assert_eq!(origen("DE-OFFICE.TTF"), Origen::Sistema);

    // Una de Windows que la app ya desactivó y volvió a activar (ahora es una
    // copia, con otro propietario) sigue siendo del sistema.
    let mut apuntadas = HashSet::new();
    apuntadas.insert(fuentes::clave_ruta(&ent.fuentes_sistema.join("a-mano.ttf")));
    fuentes::poner_de_windows_apuntadas(&ent, &apuntadas).unwrap();
    assert_eq!(origen("a-mano.ttf"), Origen::Sistema);
}

/// En un Windows de verdad, Arial es de TrustedInstaller.
#[test]
fn arial_es_de_trustedinstaller() {
    let windir = std::env::var_os("WINDIR").unwrap();
    let arial = Path::new(&windir).join("Fonts").join("arial.ttf");
    assert_eq!(tunafonts_lib::win::de_trustedinstaller(&arial), Some(true));
}

#[test]
fn desinstalar_una_de_usuario_activa_y_otra_desactivada() {
    let tmp = tempfile::tempdir().unwrap();
    let base = format!(r"Software\tuna-fonts-pruebas\des-{}", std::process::id());
    let _limpia = Limpieza(base.clone());
    let ent = Entorno {
        fuentes_sistema: tmp.path().join("sistema"),
        fuentes_usuario: tmp.path().join("usuario"),
        clave_sistema: format!(r"{base}\Sistema"),
        clave_usuario: format!(r"{base}\Fonts"),
        datos: tmp.path().join("datos"),
        tocar_windows: false,
        de_windows: |_| true,
        de_programas: HashSet::new(),
    };
    let fuente = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/GochiHand-Regular.ttf");
    let caras = leer::caras(&fuente).unwrap();
    let familia = vec!["Gochi Hand".to_string()];

    // Activa: se borra el archivo y su entrada del registro.
    let instalada = cambios::instalar_usuario(&ent, &fuente, &caras).unwrap();
    let mut cache = Cache::default();
    let entradas = fuentes::entradas(&ent, &[]);
    cache.actualizar(&entradas);
    let r = cambios::desinstalar(&ent, &entradas, &cache, &familia);
    assert_eq!((r.cambiadas, r.bloqueadas), (1, 0), "{:?}", r.errores);
    assert!(!instalada.exists());
    assert!(registro::leer(registro::hkcu(), &ent.clave_usuario).is_empty());

    // Desactivada: se borra de la carpeta de la app y de la lista de guardadas.
    cambios::instalar_usuario(&ent, &fuente, &caras).unwrap();
    let entradas = fuentes::entradas(&ent, &[]);
    cache.actualizar(&entradas);
    let mut apagadas = HashSet::new();
    cambios::alternar(&ent, &entradas, &cache, &familia, false, false, &mut apagadas);
    assert_eq!(fuentes::guardadas(&ent).len(), 1);
    let entradas = fuentes::entradas(&ent, &[]);
    cache.actualizar(&entradas);
    let r = cambios::desinstalar(&ent, &entradas, &cache, &familia);
    assert_eq!(r.cambiadas, 1, "{:?}", r.errores);
    assert!(fuentes::guardadas(&ent).is_empty());
    assert!(std::fs::read_dir(ent.desactivadas()).map(|d| d.count()).unwrap_or(0) == 0);
}

/// Con una fuente de Windows que se sabe que las trae: versalitas, números
/// antiguos y ligaduras. Si no está (otro sistema), no comprueba nada.
#[test]
fn rasgos_opentype_e_info_de_calibri() {
    let ruta = Path::new(r"C:\Windows\Fonts\calibri.ttf");
    if !ruta.is_file() {
        return;
    }
    let caras = leer::caras(ruta).unwrap();
    for r in ["smcp", "onum", "liga"] {
        assert!(caras[0].rasgos.iter().any(|x| x == r), "falta {r}: {:?}", caras[0].rasgos);
    }
    let i = leer::info(ruta).unwrap();
    assert!(i.copyright.is_some() && i.version.is_some(), "{i:?}");
    println!("{:?}\n{:?}", caras[0].rasgos, i);
}

/// El mismo estilo instalado y, además, en una carpeta vigilada: sale como duplicado.
#[test]
fn estilo_repetido_en_dos_archivos_es_duplicado() {
    let tmp = tempfile::tempdir().unwrap();
    let base = format!(r"Software\tuna-fonts-pruebas\dup-{}", std::process::id());
    let _limpia = Limpieza(base.clone());
    let ent = Entorno {
        fuentes_sistema: tmp.path().join("sistema"),
        fuentes_usuario: tmp.path().join("usuario"),
        clave_sistema: format!(r"{base}\Sistema"),
        clave_usuario: format!(r"{base}\Fonts"),
        datos: tmp.path().join("datos"),
        tocar_windows: false,
        de_windows: |_| true,
        de_programas: HashSet::new(),
    };
    let fuente = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/GochiHand-Regular.ttf");
    cambios::instalar_usuario(&ent, &fuente, &leer::caras(&fuente).unwrap()).unwrap();
    let carpeta = tmp.path().join("vigilada");
    std::fs::create_dir_all(&carpeta).unwrap();
    std::fs::copy(&fuente, carpeta.join("copia.ttf")).unwrap();

    let mut cache = Cache::default();
    cache.actualizar(&fuentes::entradas(&ent, &[carpeta.clone()]));
    let f = cache.familias.iter().find(|f| f.nombre == "Gochi Hand").unwrap();
    assert_eq!(f.estilos.len(), 1);
    assert_eq!(f.duplicados.len(), 1, "{:?}", f.duplicados);
}
