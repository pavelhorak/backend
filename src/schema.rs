table! {
	booking (id) {
		id -> Integer,
		name -> Text,
		description -> Text,
		rooms -> Integer,
		begin_time -> Text,
		end_time -> Text,
		layout -> Integer,
		approved -> Integer,
		people -> Nullable<Integer>,
	}
}

table! {
	users (id) {
		id -> Integer,
		name -> Text,
		email -> Text,
		role -> Text,
	}
}

allow_tables_to_appear_in_same_query!(booking, users,);
