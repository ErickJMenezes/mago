{
  "error": null,
  "interner": [
    [
      4,
      "<?php"
    ],
    [
      5,
      "\n\n"
    ],
    [
      10,
      "\n"
    ],
    [
      2,
      "#!/usr/bin/env php\n<?php\n\necho \"Hello, World!\\n\";\n"
    ],
    [
      9,
      ";"
    ],
    [
      7,
      " "
    ],
    [
      6,
      "echo"
    ],
    [
      8,
      "\"Hello, World!\\n\""
    ],
    [
      3,
      "#!/usr/bin/env php"
    ],
    [
      1,
      "m.php"
    ]
  ],
  "program": {
    "source": [
      1,
      "UserDefined"
    ],
    "statements": {
      "inner": [
        {
          "type": "Inline",
          "value": {
            "kind": "Shebang",
            "span": {
              "end": {
                "offset": 19,
                "source": [
                  1,
                  "UserDefined"
                ]
              },
              "start": {
                "offset": 0,
                "source": [
                  1,
                  "UserDefined"
                ]
              }
            },
            "value": 3
          }
        },
        {
          "type": "OpeningTag",
          "value": {
            "type": "Full",
            "value": {
              "span": {
                "end": {
                  "offset": 24,
                  "source": [
                    1,
                    "UserDefined"
                  ]
                },
                "start": {
                  "offset": 19,
                  "source": [
                    1,
                    "UserDefined"
                  ]
                }
              },
              "value": 4
            }
          }
        },
        {
          "type": "Echo",
          "value": {
            "echo": {
              "span": {
                "end": {
                  "offset": 30,
                  "source": [
                    1,
                    "UserDefined"
                  ]
                },
                "start": {
                  "offset": 26,
                  "source": [
                    1,
                    "UserDefined"
                  ]
                }
              },
              "value": 6
            },
            "terminator": {
              "type": "Semicolon",
              "value": {
                "end": {
                  "offset": 49,
                  "source": [
                    1,
                    "UserDefined"
                  ]
                },
                "start": {
                  "offset": 48,
                  "source": [
                    1,
                    "UserDefined"
                  ]
                }
              }
            },
            "values": {
              "inner": [
                {
                  "type": "Literal",
                  "value": {
                    "type": "String",
                    "value": {
                      "kind": {
                        "type": "DoubleQuoted"
                      },
                      "span": {
                        "end": {
                          "offset": 48,
                          "source": [
                            1,
                            "UserDefined"
                          ]
                        },
                        "start": {
                          "offset": 31,
                          "source": [
                            1,
                            "UserDefined"
                          ]
                        }
                      },
                      "value": 8
                    }
                  }
                }
              ],
              "tokens": []
            }
          }
        }
      ]
    },
    "trivia": {
      "inner": [
        {
          "kind": {
            "type": "WhiteSpace"
          },
          "span": {
            "end": {
              "offset": 26,
              "source": [
                1,
                "UserDefined"
              ]
            },
            "start": {
              "offset": 26,
              "source": [
                1,
                "UserDefined"
              ]
            }
          },
          "value": 5
        },
        {
          "kind": {
            "type": "WhiteSpace"
          },
          "span": {
            "end": {
              "offset": 31,
              "source": [
                1,
                "UserDefined"
              ]
            },
            "start": {
              "offset": 31,
              "source": [
                1,
                "UserDefined"
              ]
            }
          },
          "value": 7
        },
        {
          "kind": {
            "type": "WhiteSpace"
          },
          "span": {
            "end": {
              "offset": 50,
              "source": [
                1,
                "UserDefined"
              ]
            },
            "start": {
              "offset": 50,
              "source": [
                1,
                "UserDefined"
              ]
            }
          },
          "value": 10
        }
      ]
    }
  }
}
