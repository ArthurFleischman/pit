package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"

	"github.com/ArthurFleischman/pit/internal/models"
)

func main() {
	newCommand := flag.NewFlagSet("new", flag.ExitOnError)
	language := newCommand.String("lang", "", "set the project language (required)")
	projectName := newCommand.String("name", "", "set the project name (required)")
	projectType := newCommand.String("type", "", "set application type(required)")

	switch os.Args[1] {
	case "new":
		newCommand.Parse(os.Args[2:])
	}

	if newCommand.Parsed() {
		if *language == "" {
			fmt.Println("project language not selected")
			os.Exit(1)
		}
		if *projectName == "" {
			fmt.Println("project name not selected")
			os.Exit(1)
		}
		if *projectType == "" {
			fmt.Println("project type not selected")
			os.Exit(1)
		}
		project := models.NewProject(*projectName, *language, *projectType)
		// fmt.Println(*language)
		if err := buildProject(project); err != nil {
			fmt.Println(err.Error())
			return
		}
		fmt.Println("project built successfully")
	}

}
func readJSONFile(path string) ([]byte, error) {
	// file, err := os.OpenFile(path, os.O_RDWR|os.O_CREATE|os.O_APPEND, 0666)
	// if err != nil {
	// 	return err
	// }
	// defer file.Close()
	fileScanner, err := ioutil.ReadFile(path)
	if err != nil {
		return nil, err
	}
	return fileScanner, nil
}
func getTemplates() (*[]map[string]interface{}, error) {
	var template []map[string]interface{}
	fileBytes, err := readJSONFile(os.Getenv("HOME") + "/.templates.json")
	if err != nil {
		return nil, err
	}
	json.Unmarshal(fileBytes, &template)
	return &template, nil
}

func getAppTemplate(teamplateName string, applicationTemplate *[]map[string]interface{}) (*map[string]interface{}, error) {
	for _, value := range *applicationTemplate {
		if template, ok := value[teamplateName]; ok {
			mapParsed := template.(map[string]interface{})
			return &mapParsed, nil
		}
	}
	return nil, &models.CustomErr{Message: "language type not found, edit .templates.json"}
}

func getApplicationType(typeName string, applicationType []interface{}) (*map[string]interface{}, error) {
	for _, tp := range applicationType {
		tpParsed := tp.(map[string]interface{})
		if value, ok := tpParsed[typeName]; ok {
			mapParsed := value.(map[string]interface{})
			return &mapParsed, nil
		}
	}
	return nil, &models.CustomErr{Message: "applciation type not found, edit .templates.json"}
}

func createAppFolder(name, path string) (*string, error) {
	newPath := path + "/" + name
	if err := os.MkdirAll(newPath, os.ModePerm); err != nil {
		return &newPath, err
	}
	return &newPath, nil
}

func createAppSubFolders(path string, folders []interface{}) error {
	for _, dir := range folders {
		err := os.Chdir(path)
		if err != nil {
			return err
		}
		os.MkdirAll(dir.(string), os.ModePerm)
	}
	return nil
}
func runCommands(path string, commands []interface{}) {
	var argList []string
	for _, command := range commands {
		commandParsed := command.(map[string]interface{})
		fmt.Printf("running %s %v...\n", commandParsed["main"].(string), commandParsed["args"])
		for _, arg := range commandParsed["args"].([]interface{}) {
			argList = append(argList, arg.(string))
		}
		ex := exec.Command(commandParsed["main"].(string), argList...)
		ex.Dir = path
		ex.Stderr = os.Stdout
		if err := ex.Run(); err != nil {
			fmt.Println(err.Error())
		}
		argList = []string{}

	}
}

func buildProject(project *models.Project) error {
	var path string
	//get templates from json file
	templates, err := getTemplates()
	if err != nil {
		return err
	}
	//get selected templates if exists
	template, err := getAppTemplate(project.Language, templates)
	if err != nil {
		return err
	}
	//set language workdir
	path = (*template)["work_dir"].(string)
	//get type template
	list := (*template)["app_type"].([]interface{})

	tp, err := getApplicationType(project.Type, list)
	if err != nil {
		return err
	}
	directoryList := (*tp)["directories"].([]interface{})
	newPath, err := createAppFolder(project.Name, path)
	if err != nil {
		return err
	}
	createAppSubFolders(*newPath, directoryList)
	commandList := (*tp)["commands"].([]interface{})
	runCommands(*newPath, commandList)
	return nil
}
