// @generated automatically by Diesel CLI.

diesel::table! {
    corrupted_mementos (extrinsic_hash) {
        extrinsic_hash -> Text,
        extrinsic_id -> Integer,
        block_id -> Integer,
        block_hash -> Text,
        minted -> Integer,
    }
}
