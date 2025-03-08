import utils.dbinterface as db
import time

cursor = db.connect_to_db().cursor()
datapoints = db.read_data_range(cursor, "XAUUSD", 1738734540, 1738739550)
print(datapoints)