import psycopg2
from psycopg2 import sql
import os
from dotenv import load_dotenv

load_dotenv(dotenv_path="../.env")

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

def read_datapoint(cursor, ticker, time):
    query = sql.SQL("SELECT * FROM hist_stock_data WHERE ticker = %s AND time = to_timestamp(%s);")
    cursor.execute(query, (ticker, time))
    result = cursor.fetchone()
    if result:
        colnames = [desc[0] for desc in cursor.description]
        data = dict(zip(colnames, result))
        return data
    return None

def read_data_range(cursor, ticker, startTime, endTime):
    query = sql.SQL("SELECT * FROM hist_stock_data WHERE ticker = %s AND time >= to_timestamp(%s) AND time <= to_timestamp(%s);")
    cursor.execute(query, (ticker, startTime, endTime))
    result = cursor.fetchall()
    if result:
        colnames = [desc[0] for desc in cursor.description]
        data = [dict(zip(colnames, row)) for row in result]
        return data
    return None

def read_ticker(ticker):
    cursor = connect_to_db().cursor()
    query = sql.SQL("SELECT * FROM hist_stock_data WHERE ticker = %s;")
    cursor.execute(query, (ticker,))
    result = cursor.fetchall()
    if result:
        colnames = [desc[0] for desc in cursor.description]
        data = [dict(zip(colnames, row)) for row in result]
        return data
    return None