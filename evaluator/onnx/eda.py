# eda.py
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.read_csv('training_data/grill_sessions.csv')

# ── 1. Basic overview ─────────────────────────────────────────────────
print("Shape:", df.shape)
print("\nData types:\n", df.dtypes)
print("\nMissing values:\n", df.isnull().sum())
print("\nBasic stats:\n", df.describe())

# ── 2. Distribution of target variable ───────────────────────────────
fig, axes = plt.subplots(1, 2, figsize=(12, 4))

axes[0].hist(df['time_remaining'], bins=50, edgecolor='black')
axes[0].set_title('Distribution of time_remaining (target variable)')
axes[0].set_xlabel('Minutes remaining')
axes[0].set_ylabel('Count')

# Log transform – time_remaining is right-skewed
axes[1].hist(np.log1p(df['time_remaining']), bins=50, edgecolor='black')
axes[1].set_title('Log distribution of time_remaining')
axes[1].set_xlabel('log(minutes remaining + 1)')
plt.tight_layout()
plt.savefig('eda/target_distribution.png')
plt.show()

# ── 3. Temperature curves – do they look realistic? ──────────────────
fig, ax = plt.subplots(figsize=(12, 6))
sample_sessions = df['session_id'].unique()[:10]
for sid in sample_sessions:
    s = df[df['session_id'] == sid]
    ax.plot(s['time_elapsed'], s['temperature'], alpha=0.6)
ax.set_xlabel('Time elapsed (minutes)')
ax.set_ylabel('Temperature (°C)')
ax.set_title('Sample grilling curves')
plt.savefig('eda/curves.png')
plt.show()

# ── 4. Key insight: heating rate changes over time ────────────────────
# Compute per-session heating rates
df = df.sort_values(['session_id', 'time_elapsed'])
df['temp_diff'] = df.groupby('session_id')['temperature'].diff()
df['time_diff'] = df.groupby('session_id')['time_elapsed'].diff()
df['heating_rate'] = df['temp_diff'] / df['time_diff']

fig, ax = plt.subplots(figsize=(12, 6))
for sid in sample_sessions:
    s = df[df['session_id'] == sid].dropna()
    ax.plot(s['time_elapsed'], s['heating_rate'].rolling(3).mean(), alpha=0.6)
ax.set_xlabel('Time elapsed (minutes)')
ax.set_ylabel('Heating rate (°C/min)')
ax.set_title('Heating rate decelerates over time (key feature!)')
ax.axhline(y=0, color='red', linestyle='--')
plt.savefig('eda/heating_rate.png')
plt.show()

# ── 5. Correlation with target variable ──────────────────────────────
# What actually predicts time_remaining?
numeric_cols = ['temperature', 'target_temp', 'time_elapsed',
                'heating_rate', 'time_remaining']
corr = df[numeric_cols].dropna().corr()

fig, ax = plt.subplots(figsize=(8, 6))
sns.heatmap(corr, annot=True, fmt='.2f', cmap='coolwarm', ax=ax)
ax.set_title('Feature correlations')
plt.tight_layout()
plt.savefig('eda/correlations.png')
plt.show()

# Key findings to look for:
# - time_elapsed should correlate negatively with time_remaining (obvious)
# - heating_rate should correlate negatively with time_remaining
# - temp_remaining (target - current) should correlate strongly positively
print("\nCorrelation with time_remaining:")
print(corr['time_remaining'].sort_values(ascending=False))