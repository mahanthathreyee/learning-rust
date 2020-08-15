## Requirements and setup
1. [Rust](https://www.rust-lang.org/)
2. Internet connection with access to [https://restcountries.eu/](https://restcountries.eu/)
3. Run the following command: `cargo build && cargo run`

## Dependencies [crates.io](https://crates.io/)
* reqwest
* tokie
* url
* serde json

## Sample

```
Enter country name: egypt
Array([
    Object({
        "alpha2Code": String(
            "EG",
        ),
        "alpha3Code": String(
            "EGY",
        ),
        "altSpellings": Array([
            String(
                "EG",
            ),
            String(
                "Arab Republic of Egypt",
            ),
        ]),
        "area": Number(
            1002450.0,
        ),
        "borders": Array([
            String(
                "ISR",
            ),
            String(
                "LBY",
            ),
            String(
                "SDN",
            ),
        ]),
        "callingCodes": Array([
            String(
                "20",
            ),
        ]),
        "capital": String(
            "Cairo",
        ),
        "cioc": String(
            "EGY",
        ),
        "currencies": Array([
            Object({
                "code": String(
                    "EGP",
                ),
                "name": String(
                    "Egyptian pound",
                ),
                "symbol": String(
                    "£",
                ),
            }),
        ]),
        "demonym": String(
            "Egyptian",
        ),
        "flag": String(
            "https://restcountries.eu/data/egy.svg",
        ),
        "gini": Number(
            30.8,
        ),
        "languages": Array([
            Object({
                "iso639_1": String(
                    "ar",
                ),
                "iso639_2": String(
                    "ara",
                ),
                "name": String(
                    "Arabic",
                ),
                "nativeName": String(
                    "العربية",
                ),
            }),
        ]),
        "latlng": Array([
            Number(
                27.0,
            ),
            Number(
                30.0,
            ),
        ]),
        "name": String(
            "Egypt",
        ),
        "nativeName": String(
            "مصر\u{200e}",
        ),
        "numericCode": String(
            "818",
        ),
        "population": Number(
            91290000,
        ),
        "region": String(
            "Africa",
        ),
        "regionalBlocs": Array([
            Object({
                "acronym": String(
                    "AU",
                ),
                "name": String(
                    "African Union",
                ),
                "otherAcronyms": Array([]),
                "otherNames": Array([
                    String(
                        "الاتحاد الأفريقي",
                    ),
                    String(
                        "Union africaine",
                    ),
                    String(
                        "União Africana",
                    ),
                    String(
                        "Unión Africana",
                    ),
                    String(
                        "Umoja wa Afrika",
                    ),
                ]),
            }),
            Object({
                "acronym": String(
                    "AL",
                ),
                "name": String(
                    "Arab League",
                ),
                "otherAcronyms": Array([]),
                "otherNames": Array([
                    String(
                        "جامعة الدول العربية",
                    ),
                    String(
                        "Jāmiʻat ad-Duwal al-ʻArabīyah",
                    ),
                    String(
                        "League of Arab States",
                    ),
                ]),
            }),
        ]),
        "subregion": String(
            "Northern Africa",
        ),
        "timezones": Array([
            String(
                "UTC+02:00",
            ),
        ]),
        "topLevelDomain": Array([
            String(
                ".eg",
            ),
        ]),
        "translations": Object({
            "br": String(
                "Egito",
            ),
            "de": String(
                "Ägypten",
            ),
            "es": String(
                "Egipto",
            ),
            "fa": String(
                "مصر",
            ),
            "fr": String(
                "Égypte",
            ),
            "hr": String(
                "Egipat",
            ),
            "it": String(
                "Egitto",
            ),
            "ja": String(
                "エジプト",
            ),
            "nl": String(
                "Egypte",
            ),
            "pt": String(
                "Egipto",
            ),
        }),
    }),
])
```
