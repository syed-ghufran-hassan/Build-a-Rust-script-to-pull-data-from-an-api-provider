# Build-a-Rust-script-to-pull-data-from-an-api-provider

,The script retrieves financial data for a list of tickers from the Polygon API and exports the data to a CSV file. Here's an overview of what the script will do:

Read a list of tickers and IDs from the https://www.sec.gov/include/ticker.txt file.
For each ticker in the list, call the https://api.polygon.io/v3/reference/tickers/[ticker] and https://api.polygon.io/vX/reference/financials?ticker=[ticker] API endpoints using the Rust reqwest crate.
Parse the JSON responses and extract the required data.
Write the data to a CSV file using the Rust csv crate.
Implement an option to go through all tickers or individual tickers using the Rust clap crate.
Use the Rust dotenv crate to import and use the API key as a header.
