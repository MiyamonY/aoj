// Package main provides
//
// File:  topologicalsort.go
// Author: ymiyamoto
//
// Created on Tue Jan 29 03:12:10 2019
//
package main

import "fmt"

type graph [][]int

func (g graph) topologicalSortBFS(n int, indeg []int, visited []bool) (sorted []int) {
	q := []int{n}
	for len(q) > 0 {
		m := q[0]
		sorted = append(sorted, m)
		visited[m] = true

		q = q[1:]
		for i := range g[m] {
			to := g[m][i]
			indeg[to]--
			if indeg[to] == 0 && !visited[to] {
				q = append(q, to)
			}
		}
	}
	return sorted
}

func (g graph) topologicalSort() (sorted []int) {
	indeg := make([]int, len(g))
	for i := range g {
		for j := range g[i] {
			in := g[i][j]
			indeg[in]++
		}
	}

	visited := make([]bool, len(g))
	for i := range indeg {
		if indeg[i] == 0 && !visited[i] {
			t := g.topologicalSortBFS(i, indeg, visited)
			sorted = append(sorted, t...)
		}
	}

	return sorted
}

func main() {
	var V, E int
	fmt.Scan(&V, &E)

	g := make(graph, V)
	for i := 0; i < E; i++ {
		var s, t int
		fmt.Scan(&s, &t)
		g[s] = append(g[s], t)
	}

	sorted := g.topologicalSort()
	for _, s := range sorted {
		fmt.Println(s)
	}
}
