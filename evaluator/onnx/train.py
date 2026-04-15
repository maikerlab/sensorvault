import pandas as pd
import numpy as np
import json
from sklearn.ensemble import GradientBoostingRegressor
from sklearn.model_selection import train_test_split, cross_val_score
from sklearn.metrics import mean_absolute_error, r2_score
from sklearn.pipeline import Pipeline
from sklearn.preprocessing import StandardScaler
from skl2onnx import convert_sklearn
from skl2onnx.common.data_types import FloatTensorType

from features import engineer_features, FEATURE_COLS, TARGET_COL

# ── 1. Load and prepare data ──────────────────────────────────────────
df_raw = pd.read_csv('training_data/grill_sessions.csv')
df     = engineer_features(df_raw)

# drop rows with NaN in any feature or target column
cols_needed = FEATURE_COLS + [TARGET_COL, 'session_id']
df = df.dropna(subset=cols_needed)
# also drop rows where rates are infinite (division by zero edge case)
df = df.replace([np.inf, -np.inf], np.nan)
df = df.dropna(subset=cols_needed)
print(f"Rows after dropping NaN: {len(df)}")

X = df[FEATURE_COLS].values.astype(np.float32)
y = df[TARGET_COL].values.astype(np.float32)

print(f"Training samples: {len(X)}")
print(f"Features: {FEATURE_COLS}")
print(f"Target range: {y.min():.1f} – {y.max():.1f} minutes")

# ── 2. Train/test split ───────────────────────────────────────────────
# Split by session, not by row – to avoid data leakage
# (rows from the same session should not be in both train and test)
sessions   = df['session_id'].unique()
train_sess, test_sess = train_test_split(sessions, test_size=0.2, random_state=42)

train_mask = df['session_id'].isin(train_sess)
test_mask  = df['session_id'].isin(test_sess)

X_train, y_train = X[train_mask], y[train_mask]
X_test,  y_test  = X[test_mask],  y[test_mask]

print(f"Train: {len(X_train)}, Test: {len(X_test)}")

# ── 3. Train model ────────────────────────────────────────────────────
pipeline = Pipeline([
    ("scaler", StandardScaler()),
    ("model",  GradientBoostingRegressor(
        n_estimators=200,
        max_depth=4,
        learning_rate=0.05,
        subsample=0.8,
        random_state=42,
    )),
])

pipeline.fit(X_train, y_train)

# ── 4. Evaluate ───────────────────────────────────────────────────────
y_pred = pipeline.predict(X_test)
mae    = mean_absolute_error(y_test, y_pred)
r2     = r2_score(y_test, y_pred)

print(f"\nTest MAE:  {mae:.2f} minutes")
print(f"Test R²:   {r2:.3f}")

# Cross-validation
cv_scores = cross_val_score(
    pipeline, X_train, y_train,
    cv=5, scoring='neg_mean_absolute_error'
)
print(f"CV MAE:    {-cv_scores.mean():.2f} ± {cv_scores.std():.2f} minutes")

# Feature importance
importances = pipeline.named_steps['model'].feature_importances_
for feat, imp in sorted(zip(FEATURE_COLS, importances),
                        key=lambda x: x[1], reverse=True):
    print(f"  {feat:20s}: {imp:.3f}")

# ── 5. Export to ONNX ─────────────────────────────────────────────────
initial_type = [("float_input", FloatTensorType([None, len(FEATURE_COLS)]))]

onnx_model = convert_sklearn(
    pipeline,
    initial_types=initial_type,
    target_opset={"": 17, "ai.onnx.ml": 3},
)

with open("grill_predictor.onnx", "wb") as f:
    f.write(onnx_model.SerializeToString())

# Save feature metadata for Rust
metadata = {
    "feature_cols":  FEATURE_COLS,
    "target_col":    TARGET_COL,
    "n_features":    len(FEATURE_COLS),
    "training_mae":  float(mae),
    "training_r2":   float(r2),
}
with open("grill_predictor_meta.json", "w") as f:
    json.dump(metadata, f, indent=2)

print("\nExported grill_predictor.onnx")