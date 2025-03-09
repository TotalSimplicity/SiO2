import utils.dbinterface as db
import time

cursor = db.connect_to_db().cursor()
datapoints = db.read_data_range(cursor, "MSFT", 1736734540, 1738739550)
for datapoint in datapoints:
    print(f"{datapoint["time"]} ${datapoint["close"]}")