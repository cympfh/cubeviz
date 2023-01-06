# cubeviz

A Cube Vizualizer.

## Usage

```bash
$ cubeviz <input> > <out.svg>

$ cubeviz < samples/face_with_side.dot | convert -size 200x200 - out.png  # svg -> png
```

## Examples

A Face 3x3

```
Face {
  R O O
  W Y Y
  G B .
}
```

![](https://user-images.githubusercontent.com/2749629/184492773-98ffb3ec-c72c-457e-9392-fc90a8b4d90b.png)

A Face with its side

```
Face {
    G G B
  W R O O R
  B W Y Y O
  G G B . Y
    O G R
}
```

![](https://user-images.githubusercontent.com/2749629/184492775-ca20fb0c-335c-4745-830a-7d5bd005116e.png)

A whole cube (net style)

```
Cube {
    ...
    ...
    ...

    ... ... ... ...
    RRG RGG OOO BBB
    RRR GGG OOO BBB

    WWW
    WWW
    WWW
}
```

![](https://user-images.githubusercontent.com/2749629/210928081-0a7f7612-29ee-4271-bb38-1492339eeb15.png)

## Colors

- `W`, White
- `Y`, Yello
- `R`, Red
- `O`, Orange
- `B`, Blue
- `G`, Green
- `.`, Masked cube
