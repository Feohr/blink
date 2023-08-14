# blink

ASCII animation of a blinking eye.

## Explanation

The following animation uses pythagorean theorem to deduce if a given point is inside the ellipse.
This ellipse's major axis is varied over time to give the illusion of a blinking eye. Two rows of
ellipse are skipped to give the ellipse a more eye-like look. Lastly, we clip a static circle
inside the ellipse to act like an iris.

- The equation for eye ball:

    ```(x - k)^2 / a^2 + (y - h)^2 / b^2 < 1```

- The equation for the iris:

    ```(x - k)^2 + (y - h)^2 < radius^2```

## Usage

Clone the repository and then run in the terminal:

```bash
$ cargo run
```

## Output
```bash
                        █████████████████████████
                █████████████████       █████████████████
           ██████████████████               ██████████████████
       ███████████████████                     ███████████████████
     ████████████████████                       ████████████████████
   ████████████████████                           ████████████████████
   ████████████████████                           ████████████████████
     ████████████████████                       ████████████████████
       ███████████████████                     ███████████████████
           ██████████████████               ██████████████████
                █████████████████       █████████████████
                        █████████████████████████
```
