import gymnasium as gym
import gym_trading_env
import utils.dbinterface as db
import pandas as pd

rawData = db.read_ticker("XAUUSD")
df = pd.DataFrame(rawData)

df["feature_close"] = df["close"].pct_change()

df["feature_open"] = df["open"]/df["close"]

df["feature_high"] = df["high"]/df["close"]

df["feature_low"] = df["low"]/df["close"]

df["feature_volume"] = df["volume"] / df["volume"].rolling(7*24).max()

df.rename(columns={"percent_K": "feature_k"}, inplace=True)
df.rename(columns={"percent_D": "feature_d"}, inplace=True)
df.rename(columns={"signal_line": "feature_signal"}, inplace=True)

df.sort_index(inplace= True)
df.dropna(inplace= True)
df.drop_duplicates(inplace=True)
print(df.head())

env = gym.make("TradingEnv",
        name= "BTCUSD",
        df = df,
        positions = [ -1, 0, 1],
        trading_fees = 0.01/100, 
        borrow_interest_rate= 0.0003/100,
    )
