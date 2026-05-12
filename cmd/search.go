/*
Copyright © 2026 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
	"os/exec"
)

// searchCmd represents the search command
var searchCmd = &cobra.Command{
	Use:   "search",
	Short: "Search for a player's stats",
	Long:  `Search for either all the stats of a player or individual stats.`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("search called")
		pyCmd := exec.Command("python3", "-c", "from", "modules", "import", "data;", fmt.Sprintf("data.select_player(%s)", args[0]))
		out, err := pyCmd.CombinedOutput()
		if err != nil {
			fmt.Println("Error executing Python code:", err)
			return
		}
		fmt.Println(string(out))
	},
}

func init() {
	rootCmd.AddCommand(searchCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// searchCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// searchCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
