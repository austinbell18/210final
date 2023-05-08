# 210final
Calculating Betweenness Centrality with BFS

Austin Bellis
Professor Leonidas Kontothanassis
DS 210
4 May 2023
Final Project Write-Up
I chose the Email Enron dataset found at https://snap.stanford.edu/data/email-Enron.html.
The dataset has 2 columns of 367662 lines each. The nodes in this dataset represent members of
a company, and the edges are emails between those members. Initially, I was planning to use
either breadth-first search or depth-first search, and, after a bit of research, I decided to attempt
to find the betweenness centrality values of each node in order to determine the most influential
members of the company.
First, I decided to implement the breadth-first search algorithm to perform a breadth-first
search from a given starting point that will return the distance traveled for each path taken. Then,
in a separate function, a loop performs a breadth-first search on every node and uses the return
values to calculate the betweenness centrality of each node and normalizes the values. Finally,
the values are sorted by the centralities in descending order.
I was hoping to get an idea of how information moves in the company, and I would say I
did. While not surprising, it seems information moves top-down in a pyramid format. For a given
node, there are fewer senders to it than who it sends to. The lower number nodes (closer to 0)
likely do not communicate with many higher nodes, but expected messages to be shared
top-down.
So, given what the project does, all one needs to run the code is the path the file is found
on. In my current code, I have the path name in the read_file function, but the code could be
changed to make the code more versatile. The path name could be a variable, but as it stands,
running the code with “--release” should execute in under a minute, and the output will give the
top 10 nodes with the greatest centrality values.
There are two tests. One tests the betweenness centrality calculation, and the other tests
the sorting function. The main, tests, graph, and read functions are all split into separate modules
as well.
