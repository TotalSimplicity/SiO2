import pandas as pd
import psycopg2
from psycopg2 import sql
import time
from dotenv import load_dotenv
import os

load_dotenv()

def connect_to_db():
    try:
        connection = psycopg2.connect(
            host=os.getenv("DB_HOST"),
            dbname=os.getenv("POSTGRES_DB"),
            user=os.getenv("POSTGRES_USER"),
            password=os.getenv("POSTGRES_PASSWORD")
        )
        return connection
    except Exception as e:
        print("Error connecting to the database:", e)
        return None

def create_table(cursor):
    create_table_query = """
    CREATE TABLE IF NOT EXISTS hist_stock_data (
        time TIMESTAMPTZ NOT NULL,
        ticker TEXT NOT NULL,
        open DOUBLE PRECISION,
        high DOUBLE PRECISION,
        low DOUBLE PRECISION,
        close DOUBLE PRECISION,
        volume BIGINT,
        "percent_K" DOUBLE PRECISION,
        "percent_D" DOUBLE PRECISION,
        normalized_vol DOUBLE PRECISION,
        normalized_price DOUBLE PRECISION,
        rrof_smooth DOUBLE PRECISION,
        signal_line DOUBLE PRECISION,
        eom_pos DOUBLE PRECISION,
        eom_neg DOUBLE PRECISION,
        compression_pos DOUBLE PRECISION,
        compression_neg DOUBLE PRECISION,
        PRIMARY KEY (time, ticker)
    );
    """
    cursor.execute(create_table_query)

def upload_csv_to_db(csv_file_path, ticker, cursor):
    df = pd.read_csv(csv_file_path)
    
    df['time'] = pd.to_datetime(df['time'], unit='s')

    insert_query = """
    INSERT INTO hist_stock_data (
        time, ticker, open, high, low, close, volume, "percent_K", "percent_D", 
        normalized_vol, normalized_price, rrof_smooth, signal_line, 
        eom_pos, eom_neg, compression_pos, compression_neg
    ) VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s)
    ON CONFLICT (time, ticker) DO NOTHING;
    """
    
    for index, row in df.iterrows():
        eom_pos = None if pd.isna(row['EoM +ve']) else row['EoM +ve']
        eom_neg = None if pd.isna(row['EoM -ve']) else row['EoM -ve']
        compression_pos = None if pd.isna(row['Compression +ve']) else row['Compression +ve']
        compression_neg = None if pd.isna(row['Compression -ve']) else row['Compression -ve']
        
        cursor.execute(insert_query, (
            row['time'], ticker, row['open'], row['high'], row['low'], 
            row['close'], row['Volume'], row['%K'], row['%D'], 
            row['Normalized Vol'], row['Normalized Price'], row['RROF Smooth'], 
            row['Signal Line'], eom_pos, eom_neg, 
            compression_pos, compression_neg
        ))

def main():
    ticker = input("Enter the ticker symbol: ")
    csv_file_path = input(f"Enter the path to the CSV file for {ticker}: ")

    connection = connect_to_db()
    if connection is None:
        return

    cursor = connection.cursor()

    create_table(cursor)

    try:
        upload_csv_to_db(csv_file_path, ticker, cursor)
        print("Data uploaded successfully!")
        connection.commit()
    except Exception as e:
        print("Error uploading data:", e)
        connection.rollback()
    finally:
        cursor.close()
        connection.close()

if __name__ == "__main__":
    main()