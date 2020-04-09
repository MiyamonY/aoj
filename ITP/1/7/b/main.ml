(* open Batteries
 * open BatPrintf *)

let scan fmt f =
  Scanf.sscanf (read_line ()) fmt f

(* let scan_list cnv =
 *   read_line ()
 *   |> String.split_on_char ' '
 *   |> List.map cnv *)

let rec zip xs ys =
  match xs, ys with
  | [], _ -> []
  | _, [] -> []
  | x::xs, y::ys ->
    (x, y) :: zip xs ys

let rec combination n = function
  | [] -> if n = 0 then [[]] else []
  | x::xs ->
    List.map (fun l -> x::l) (combination (n-1) xs) @ combination n xs

let dbg x =
  (* printf "[dbg]%s\n" @@ dump x; *)
  x

let rec range n m =
  if n = m then [m]
  else n :: range (n+1) m

let rec sum  = function
  | [] -> 0
  | x::xs -> x + sum xs


let rec solve () =
  let (n, x) = dbg @@ scan "%d %d" (fun a b -> (a,b))(* Tuple.Tuple2.make *) in
  let rec aux a b c =
    (if a + b + c = x then 1
     else 0) +
    if c >= n then
      if b >= n - 1 then
        if a >= n - 2 then 0
        else aux (a+1) (a+1+1) (a+1+1+1)
      else
        aux a (b+1) (b+1+1)
    else
      aux a b (c+1)
  in
  if not (n == 0 && x == 0) then
    (Printf.printf "%d\n" @@ aux 1 2 3;
     solve ())

let () =
  solve ()
