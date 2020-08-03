# pit aka project interactive terminal


## installation
`go get github.com/ArthurFleichman/pit`
`cd ~/.go/src/github.com/ArthurFleischman/pit`
`go install .`

ps: you should have GOBIN in your path

ps2: you should have a file called templates.json in your home file

## commands

`new`

## flags

`--name` define project name
`--lang` define project language
`--type` define project type (api,webapp ...)

## .template.json format

```json
[
	{
	"Go": {
		"work_dir": "/home/arthur/go/src/github.com/ArthurFleischman",
			"app_type": [
			{
				"api": {
					"directories": [
						"db/config",
					"models",
					"handlers",
					"src"
					],
					"commands": [
					{
						"main": "touch",
						"args": [
							"main.go"
						]
					},
					{
						"main": "touch",
						"args": [
							"Dockerfile"
						]
					},
					{
						"main": "go",
						"args": [
							"mod",
						"init"
						]
					}
					]
				}
			},
			{
				"webApp": {
					"directories": [
						"models",
					"handlers",
					"src",
					"views",
					"assets/images",
					"assets/fonts"
					],
					"commands": [
					{
						"main": "touch",
						"args": [
							"main.go"
						]
					},
					{
						"main": "touch",
						"args": [
							"Dockerfile"
						]
					},
					{
						"main": "go",
						"args": [
							"mod",
						"init"
						]
					}
					]
				}
			}
		]
	}
	}
]
```
