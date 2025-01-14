# Rust macros

test for macros

DeriveInput {
    attrs: [
        Attribute {
            pound_token: Pound,
            style: AttrStyle::Outer,
            bracket_token: Bracket,
            meta: Meta::List {
                path: Path {
                    leading_colon: None,
                    segments: [
                        PathSegment {
                            ident: Ident {
                                ident: "allow",
                                span: #0 bytes(32..37),
                            },
                            arguments: PathArguments::None,
                        },
                    ],
                },
                delimiter: MacroDelimiter::Paren(
                    Paren,
                ),
                tokens: TokenStream [
                    Ident {
                        ident: "unused",
                        span: #0 bytes(38..44),
                    },
                ],
            },
        },
    ],
    vis: Visibility::Inherited,
    ident: Ident {
        ident: "Direction",
        span: #0 bytes(86..95),
    },
    generics: Generics {
        lt_token: None,
        params: [],
        gt_token: None,
        where_clause: None,
    },
    data: Data::Enum {
        enum_token: Enum,
        brace_token: Brace,
        variants: [
            Variant {
                attrs: [],
                ident: Ident {
                    ident: "Up",
                    span: #0 bytes(102..104),
                },
                fields: Fields::Unnamed {
                    paren_token: Paren,
                    unnamed: [
                        Field {
                            attrs: [],
                            vis: Visibility::Inherited,
                            mutability: FieldMutability::None,
                            ident: None,
                            colon_token: None,
                            ty: Type::Path {
                                qself: None,
                                path: Path {
                                    leading_colon: None,
                                    segments: [
                                        PathSegment {
                                            ident: Ident {
                                                ident: "DirectionUp",
                                                span: #0 bytes(105..116),
                                            },
                                            arguments: PathArguments::None,
                                        },
                                    ],
                                },
                            },
                        },
                    ],
                },
                discriminant: None,
            },
            Comma,
            Variant {
                attrs: [],
                ident: Ident {
                    ident: "Down",
                    span: #0 bytes(123..127),
                },
                fields: Fields::Unit,
                discriminant: None,
            },
            Comma,
            Variant {
                attrs: [],
                ident: Ident {
                    ident: "Left",
                    span: #0 bytes(133..137),
                },
                fields: Fields::Unnamed {
                    paren_token: Paren,
                    unnamed: [
                        Field {
                            attrs: [],
                            vis: Visibility::Inherited,
                            mutability: FieldMutability::None,
                            ident: None,
                            colon_token: None,
                            ty: Type::Path {
                                qself: None,
                                path: Path {
                                    leading_colon: None,
                                    segments: [
                                        PathSegment {
                                            ident: Ident {
                                                ident: "u32",
                                                span: #0 bytes(138..141),
                                            },
                                            arguments: PathArguments::None,
                                        },
                                    ],
                                },
                            },
                        },
                    ],
                },
                discriminant: None,
            },
            Comma,
            Variant {
                attrs: [],
                ident: Ident {
                    ident: "Right",
                    span: #0 bytes(148..153),
                },
                fields: Fields::Unnamed {
                    paren_token: Paren,
                    unnamed: [
                        Field {
                            attrs: [],
                            vis: Visibility::Inherited,
                            mutability: FieldMutability::None,
                            ident: None,
                            colon_token: None,
                            ty: Type::Path {
                                qself: None,
                                path: Path {
                                    leading_colon: None,
                                    segments: [
                                        PathSegment {
                                            ident: Ident {
                                                ident: "u32",
                                                span: #0 bytes(154..157),
                                            },
                                            arguments: PathArguments::None,
                                        },
                                    ],
                                },
                            },
                        },
                        Comma,
                        Field {
                            attrs: [],
                            vis: Visibility::Inherited,
                            mutability: FieldMutability::None,
                            ident: None,
                            colon_token: None,
                            ty: Type::Path {
                                qself: None,
                                path: Path {
                                    leading_colon: None,
                                    segments: [
                                        PathSegment {
                                            ident: Ident {
                                                ident: "u32",
                                                span: #0 bytes(159..162),
                                            },
                                            arguments: PathArguments::None,
                                        },
                                    ],
                                },
                            },
                        },
                    ],
                },
                discriminant: None,
            },
            Comma,
        ],
    },
}
