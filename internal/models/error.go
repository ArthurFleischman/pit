package models

//CustomErr :
type CustomErr struct {
	Message string
}

func (err *CustomErr) Error() string {
	return err.Message
}
