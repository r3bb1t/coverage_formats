use coverage_formats::jacoco::JacocoReport;

fn main() {
    let r = std::include_bytes!("./files/kafka_coverage.exec");
    let decoded = JacocoReport::from_read(&mut &r[..]).unwrap();

    eprintln!("{decoded}");
}
