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
		if len(args) < 1 {
			fmt.Println("Error: Player tag is required")
			return
		}

		statFlag, err := cmd.Flags().GetString("stat")
		if err != nil {
			return
		}

		var pyScript string
		if statFlag != "" {
			pyScript = fmt.Sprintf("from modules import data; data.print_stat(data.get_stat('%s','%s'))", args[0], statFlag)
		} else {
			pyScript = fmt.Sprintf("from modules import data; data.print_stats(data.select_player('%s'))", args[0])
		}

		pyCmd := exec.Command("python3", "-c", pyScript)
		pyCmd.Dir = "/home/cody/workspaces/github/CodyBense/cdl-stats"

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
	searchCmd.Flags().StringP("stat", "s", "", "Search a specific stat")
}
