module ElmUser exposing (..)

type alias ElmUser =
	{ foo: Maybe(List Int)
	, id: List(Dict String(List User))
	, vector: List Int
	}
