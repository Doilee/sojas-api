// @generated automatically by Diesel CLI.

diesel::table! {
    api_cache (id) {
        id -> Unsigned<Integer>,
        api -> Varchar,
        query -> Varchar,
        result -> Json,
        created_at -> Timestamp,
    }
}

diesel::table! {
    badges (id) {
        id -> Unsigned<Integer>,
        label -> Varchar,
        description -> Text,
        color -> Varchar,
        image_url -> Nullable<Varchar>,
        priority -> Tinyint,
    }
}

diesel::table! {
    categories (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        slug -> Varchar,
        image_url -> Nullable<Varchar>,
        image_srcset -> Nullable<Text>,
    }
}

diesel::table! {
    comments (id) {
        id -> Unsigned<Integer>,
        food_id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        parent_id -> Nullable<Unsigned<Integer>>,
        body -> Text,
        likes -> Unsigned<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    daily_values (id) {
        id -> Unsigned<Bigint>,
        nutrient_id -> Unsigned<Integer>,
        amount -> Unsigned<Double>,
        unit -> Varchar,
    }
}

diesel::table! {
    food (id) {
        id -> Unsigned<Integer>,
        category_id -> Nullable<Unsigned<Integer>>,
        name -> Varchar,
        slug -> Varchar,
        views -> Unsigned<Integer>,
        description -> Nullable<Text>,
        wiki_redirects_to -> Nullable<Varchar>,
        image_url -> Nullable<Varchar>,
        image_srcset -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    media (id) {
        id -> Unsigned<Bigint>,
        model_type -> Varchar,
        model_id -> Unsigned<Bigint>,
        uuid -> Nullable<Char>,
        collection_name -> Varchar,
        name -> Varchar,
        file_name -> Varchar,
        mime_type -> Nullable<Varchar>,
        disk -> Varchar,
        conversions_disk -> Nullable<Varchar>,
        size -> Unsigned<Bigint>,
        manipulations -> Json,
        custom_properties -> Json,
        generated_conversions -> Json,
        responsive_images -> Json,
        order_column -> Nullable<Unsigned<Integer>>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    migrations (id) {
        id -> Unsigned<Integer>,
        migration -> Varchar,
        batch -> Integer,
    }
}

diesel::table! {
    nutrients (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        unit -> Varchar,
        color -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    nutrition_facts (id) {
        id -> Unsigned<Integer>,
        preparation_id -> Unsigned<Integer>,
        nutrient_id -> Unsigned<Integer>,
        value -> Double,
        min -> Double,
        max -> Double,
        lab_test_count -> Unsigned<Integer>,
        unit -> Varchar,
        ranking -> Nullable<Unsigned<Integer>>,
    }
}

diesel::table! {
    password_resets (email) {
        email -> Varchar,
        token -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    personal_access_tokens (id) {
        id -> Unsigned<Bigint>,
        tokenable_type -> Varchar,
        tokenable_id -> Unsigned<Bigint>,
        name -> Varchar,
        token -> Varchar,
        abilities -> Nullable<Text>,
        last_used_at -> Nullable<Timestamp>,
        expires_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    preparations (id) {
        id -> Unsigned<Integer>,
        food_id -> Nullable<Unsigned<Integer>>,
        usda_category_id -> Nullable<Unsigned<Integer>>,
        scientific_name -> Nullable<Varchar>,
        clean_description -> Nullable<Varchar>,
        usda_description -> Varchar,
    }
}

diesel::table! {
    sources (id) {
        id -> Unsigned<Integer>,
        sourcable_id -> Unsigned<Integer>,
        sourcable_type -> Varchar,
        name -> Varchar,
        source_id -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    suggestions (id) {
        id -> Unsigned<Integer>,
        subject -> Varchar,
        name -> Varchar,
        email -> Varchar,
        message -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    usda_categories (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        slug -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Integer>,
        email -> Varchar,
        email_verified_at -> Nullable<Timestamp>,
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(comments -> food (food_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(daily_values -> nutrients (nutrient_id));
diesel::joinable!(food -> categories (category_id));
diesel::joinable!(nutrition_facts -> nutrients (nutrient_id));
diesel::joinable!(nutrition_facts -> preparations (preparation_id));
diesel::joinable!(preparations -> food (food_id));
diesel::joinable!(preparations -> usda_categories (usda_category_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_cache,
    badges,
    categories,
    comments,
    daily_values,
    food,
    media,
    migrations,
    nutrients,
    nutrition_facts,
    password_resets,
    personal_access_tokens,
    preparations,
    sources,
    suggestions,
    usda_categories,
    users,
);
