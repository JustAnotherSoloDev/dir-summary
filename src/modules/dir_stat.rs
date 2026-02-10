

use anyhow::{Result,Context};
use csv::Writer;
use std::{
    collections::HashMap,
    env, fs,
    path::{self, PathBuf, absolute},
};
use uuid::Uuid;
use walkdir::WalkDir;




pub fn get_report_name(file_name:&str) -> Result<PathBuf> {
    let current_dir = env::current_dir()?;
    let output_dir = current_dir.join("result");
    fs::create_dir_all(&output_dir)?;
    let guid = Uuid::new_v4().to_string();
    let output_file = output_dir.join(format!("{}-{}.csv", file_name,&guid));
    return Ok(output_file);
}


pub fn create_report_for_dir(target_dir: &str)->Result<String>{
     let report_name=get_report_name("report")?;
     let file_path=path::absolute(&report_name)?;
     let file_path=file_path.to_string_lossy().to_string();
     create_report(target_dir,&report_name)?;
     return Ok(file_path)
}


pub fn create_report(target_dir: &str, report_path: &PathBuf) -> Result<()> {
    let mut wtr = Writer::from_path(report_path).context("Failed to create CSV writer")?;
    wtr.write_record(&["Filename", "Size (bytes)","Size","extension"])
        .context("Failed to write header to CSV")?;
    // wtr.write_record(&[target_dir,"",""])?;
    // wtr.flush()?;
   let target_dir=path::absolute(target_dir)?;
    for entry in WalkDir::new(target_dir).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            let file_path=entry.path();
            let extension=file_path.extension()
            .map(|ext| ext.to_string_lossy().to_string())
            .unwrap_or("".to_string());
            let file_name = path::absolute(file_path)?.to_string_lossy().to_string();
            let file_size = fs::metadata(file_path)
                .context(format!("Failed to read metadata for {}", file_name))?
                .len();
            let readable_size=bytesize::ByteSize(file_size);
            wtr.write_record(&[file_name, file_size.to_string(),readable_size.to_string(),extension])
                .context("Failed to write record to CSV")?;
        }
    }
    wtr.flush().context("Failed to flush CSV writer")?;
    Ok(())
}

pub fn directory_summary(report_path: &str, target_dir: &str) -> Result<(u64, u64)> {
    let mut total_size = 0;
    let mut total_files = 0;
    let target_dir = path::absolute(target_dir).context("invalid directory to summarize")?;
    let target_dir = target_dir
        .as_os_str()
        .to_str()
        .context("There was an error while converting target dir path to string")?;

    let report_path=path::absolute(report_path).context("invalid report path")?;
    let report_path=report_path.as_os_str().to_str()
    .context("Unable to convert report path to string")?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(report_path)
        .context("Failed to open the report CSV")?;

    for result in rdr.records() {
        let record = result.context("Failed to read record from CSV")?;
        let dir_name: String = record.get(0).unwrap_or("").to_string();
        if !dir_name.contains(target_dir) {
            continue;
        }
        let size: u64 = record
            .get(1)
            .unwrap_or("0")
            .parse()
            .context("Failed to parse size")?;
        total_size += size;
        total_files += 1;
    }

    Ok((total_size, total_files))
}

pub fn summary(report_path: &PathBuf) -> Result<String> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true) 
        .from_path(report_path)?;

    // Create a HashMap to store directory sizes
    let mut dir_sizes: HashMap<String, (u64,u64)> = HashMap::new();

    // Read the CSV file
    for result in rdr.records() {
        let record = result?;
            let path = &record[0];
            let size: u64 = record[1].parse().unwrap_or(0);

            // Get the directory path by removing the file name
            if let Some(parent_dir) = std::path::Path::new(path).parent() {
                let dir_path = parent_dir.to_string_lossy().to_string();

                // Sum the size in the corresponding directory
                let entry=*dir_sizes.entry(dir_path.clone()).or_insert((0,0));
                let new_size=entry.0+size;
                let new_total=entry.1+1;
                dir_sizes.insert(dir_path,(new_size,new_total));
            }
    }
    let summary_report=get_report_name("summary")?;
 // Create a CSV writer to output the results
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true) // Add headers to the output CSV
        .from_path(&summary_report)?;
    
    // Write the header
    wtr.write_record(&["Directory", "Size in Bytes","Total Size","Total Files"])?;

    // Write the total size of each directory into the output CSV
    for (dir, details) in dir_sizes {
        let size=details.0;
        let total_files=details.1;
        let readable_size=bytesize::ByteSize(size);
        wtr.write_record(&[dir, size.to_string(),readable_size.to_string(),total_files.to_string()])?;
    }

    // Flush the writer
    wtr.flush()?;
    let out_path=path::absolute(summary_report)?.to_string_lossy().to_string();
    Ok(out_path)
}



pub fn print(report_path: &PathBuf)->Result<()>{
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false) 
        .from_path(report_path)?;

    // Iterate over each record
    for result in rdr.records() {
        let record = result?;
        let values: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        println!("{}", values.join(", "));
    }
    Ok(())
}