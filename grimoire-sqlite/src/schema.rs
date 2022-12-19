// @generated automatically by Diesel CLI.

diesel::table! {
    ingredients (name) {
        name -> Text,
        lore -> Nullable<Text>,
        al_weight -> Integer,
        dh -> Nullable<Double>,
        dp -> Nullable<Double>,
        mdh -> Nullable<Double>,
        mdp -> Nullable<Double>,
        hot -> Nullable<Double>,
        pot -> Nullable<Double>,
        mhot -> Nullable<Double>,
        mpot -> Nullable<Double>,
        hl -> Nullable<Double>,
        pl -> Nullable<Double>,
        mhl -> Nullable<Double>,
        mpl -> Nullable<Double>,
        a -> Nullable<Double>,
        ma -> Nullable<Double>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    lores (name) {
        name -> Text,
        effectiveness -> Nullable<Double>,
        parent -> Nullable<Text>,
        parent2 -> Nullable<Text>,
    }
}

diesel::table! {
    player_character_lores (character, lore) {
        character -> Text,
        lore -> Text,
        value -> Integer,
    }
}

diesel::table! {
    player_characters (name) {
        name -> Text,
        advanced_potion_making -> Integer,
        alvarin_clade -> Bool,
    }
}


diesel::allow_tables_to_appear_in_same_query!(
    ingredients,
    lores,
    player_character_lores,
    player_characters,
);
