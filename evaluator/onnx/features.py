import pandas as pd
import numpy as np

def engineer_features(df: pd.DataFrame, window: int = 5) -> pd.DataFrame:
    """
    Create features for each reading based on session history so far.
    window = number of recent readings to compute recent_rate from.
    """
    df = df.sort_values(['session_id', 'time_elapsed']).copy()

    # Per-reading features
    df['temp_remaining'] = df['target_temp'] - df['temperature']

    # Rate features – computed per session
    df['temp_diff'] = df.groupby('session_id')['temperature'].diff()
    df['time_diff'] = df.groupby('session_id')['time_elapsed'].diff()

    # fillna(0) instead of leaving NaN for first row of each session
    df['inst_rate'] = (df['temp_diff'] / df['time_diff'].replace(0, np.nan)).fillna(0)

    # Average rate from session start
    df['avg_rate'] = df.groupby('session_id').apply(
        lambda g: g['temperature'].diff().fillna(0).cumsum() / g['time_elapsed'].clip(lower=0.01)
    ).reset_index(level=0, drop=True)

    # Recent rate – average over last N readings
    df['recent_rate'] = df.groupby('session_id')['inst_rate'] \
        .transform(lambda x: x.rolling(window, min_periods=1).mean())

    # Rate trend – is heating accelerating or decelerating?
    df['rate_trend'] = df.groupby('session_id')['inst_rate'] \
        .transform(lambda x: x.rolling(window, min_periods=2).apply(
            lambda v: np.polyfit(range(len(v)), v, 1)[0] if len(v) > 1 else 0
        ))

    # Progress ratio – how far through the cook are we?
    df['progress_ratio'] = 1 - (df['temp_remaining'] / (df['target_temp'] - 20).clip(lower=1))

    # Drop rows without enough history
    df = df.dropna(subset=['recent_rate', 'avg_rate'])
    df = df[df['time_elapsed'] > 1]  # need at least 1 minute of data

    return df

FEATURE_COLS = [
    'temperature',      # current temp
    'target_temp',      # desired final temp
    'temp_remaining',   # how far to go
    'time_elapsed',     # how long grilling so far
    'avg_rate',         # overall heating rate
    'recent_rate',      # recent heating rate
    'rate_trend',       # accelerating or decelerating?
    'progress_ratio',   # 0-1, how close to done
]

TARGET_COL = 'time_remaining'