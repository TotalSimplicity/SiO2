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

def get_ticker_stats():
    """Get statistics about available ticker data"""
    connection = connect_to_db()
    if connection is None:
        return None
        
    cursor = connection.cursor()
    try:
        query = """
        SELECT 
            ticker,
            COUNT(*) as data_points,
            MIN(time) as earliest_date,
            MAX(time) as latest_date,
            (MAX(time) - MIN(time)) as date_range
        FROM hist_stock_data
        GROUP BY ticker
        ORDER BY data_points DESC;
        """
        cursor.execute(query)
        results = cursor.fetchall()
        
        # Convert to list of dicts for easier handling
        stats = []
        for row in results:
            stats.append({
                'ticker': row[0],
                'data_points': row[1],
                'earliest_date': row[2],
                'latest_date': row[3],
                'date_range': row[4]
            })
        return stats
    finally:
        cursor.close()
        connection.close()