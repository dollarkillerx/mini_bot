package main

import (
	"fmt"
	"log"

	"github.com/Pow-Duck/processes"
	"github.com/dollarkillerx/erguotou"
)

type Cmd struct {
	Cmd string `json:"cmd"`
}

func main() {
	app := erguotou.New()

	app.Post("/cmd", func(ctx *erguotou.Context) {
		var cmd Cmd
		err := ctx.BindJson(&cmd)
		if err != nil {
			ctx.String(401, "401")
			return
		}

		go func() {
			r, err := processes.RunCommand(cmd.Cmd)
			if err != nil {
				log.Println(err)
			}

			fmt.Println(r)
		}()

		ctx.String(200, "success")
	})

	if err := app.Run(erguotou.SetHost("0.0.0.0:9781")); err != nil {
		log.Fatalln(err)
	}
}
