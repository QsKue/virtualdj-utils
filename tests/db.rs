#[test]
#[ignore]
fn test_db() {
    // TODO: use better test
    let db = virtualdj_utils::database::parse_database(r#"D:\VirtualDJ\database.xml"#).unwrap();
    virtualdj_utils::database::write_database(r#"D:\VirtualDJ\database.gen.xml"#, &db).unwrap();
}

#[test]
fn test_m3u() {

    let m3u_str = r#"#EXTVDJ:<artist>Neon Trees</artist><title>Midnight Echo</title><songlength>212.45</songlength>
C:\Music\Set1\neon_trees_midnight_echo.mp3
#EXTVDJ:<artist>Glass Animals</artist><title>Heat Waves</title><songlength>238.12</songlength>
C:\Music\Set1\glass_animals_heat_waves.mp3
#EXTVDJ:<artist>Tame Impala</artist><title>Borderline</title><songlength>215.77</songlength>
C:\Music\Set2\tame_impala_borderline.mp3
C:\Music\Misc\unknown_track.mp3
"#;

    let m3u = virtualdj_utils::m3u::parse_m3u_str(m3u_str);
    let result = virtualdj_utils::m3u::m3u_to_string(&m3u);

    assert_eq!(m3u_str, result)
}
