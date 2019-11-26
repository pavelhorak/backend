table! {
	booking (id) {
		id -> Int4,
		name -> Varchar,
		description -> Varchar,
		rooms -> Int4,
		begin_time -> Time,
		end_time -> Time,
		layout -> Int4,
		approved -> Bool,
	}
}
