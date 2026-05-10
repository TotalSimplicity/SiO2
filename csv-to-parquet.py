import sys

import pandas as pd


def csv_to_parquet(csv_file: str):
    parquet_file = csv_file.rsplit(".", 1)[0] + ".parquet"

    df = pd.read_csv(csv_file)
    df.to_parquet(parquet_file, index=False)

    print(f"Saved: {parquet_file} ({len(df)} rows)")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python convert.py <file.csv>")
        sys.exit(1)

    csv_to_parquet(sys.argv[1])
