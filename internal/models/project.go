package models

//Project structure
type Project struct {
	Name     string
	Language string
	Type     string
}

//NewProject return a new project
func NewProject(name, lang, tp string) *Project {
	return &Project{Name: name, Language: lang, Type: tp}
}
