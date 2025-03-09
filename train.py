import gymnasium as gym
import utils.dbinterface as db
import pandas as pd

rawData = db.read_ticker("XAUUSD")
df = pd.DataFrame(rawData)


print(df.head())

