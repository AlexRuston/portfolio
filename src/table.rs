use mysql::*;

pub fn print_table(results: Vec<Row>) {
    // Iterate over the results and print each row
    for row in results {
        // You can fetch values by column index or column name
        let id: i32 = row.get("id").unwrap_or_default();
        let name: String = row.get("name").unwrap_or_default();
        let description: String = row.get("description").unwrap_or_default();
        let status: String = row.get("status").unwrap_or_default();

        println!("\n=========================================\nID: {}\nName: {}Description: {}Status: {}\n=========================================\n", id, name, description, status);
    }
}