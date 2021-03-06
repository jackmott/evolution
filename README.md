# Evolving Art - Work In Progress
A desktop app that lets you generate and evolve your own art in the form of pictures or videos.
This generates random expressions, and lets the user breed them. Images are evaluated using a stack machine with SIMD instructions, leveraging [SIMDeez](https://github.com/jackmott/simdeez)
and Rayon for parallelism at the instruction level and over multiple cores.

## Some Possibilities

### HSV Images
```lisp
( HSV    
    ( - ( Cell2 Y X 0.84070134 ) ( - X ( Sqrt Y ) ) )     
    ( Cell2 ( + ( - X -0.52474713 ) ( Abs X ) ) ( + Y ( Atan2 Y ( Log 0.8803401 ) ) ) ( Abs ( Sqrt ( FBM X ( Cell1 0.10496092 Y Y ) -0.10098362 ) ) ) )    
    ( FBM ( * -0.73565507 Y ) ( Cell1 Y Y X ) ( Abs X ) ) )
 ```

![Sample Image](/samples/hsv_noise.png)

### Monochrome Images
```lisp
( Mono    
    ( FBM ( FBM 0.69943047 X ( Ridge -0.4082718 Y ( Abs ( Atan X ) ) ) ) ( Atan2 ( Log ( Sqrt ( Turbulence Y X X ) ) ) ( FBM ( - ( Ridge Y ( Cell2 Y X Y ) Y ) -0.7674043 ) ( Sqrt -0.81428957 ) -0.43793464 ) ) ( Cell1 ( - 0.4862821 0.66654444 ) ( Ridge Y Y Y ) ( FBM X Y X ) ) ) )
```

![Sample Image](/samples/bw_noise.png)

### RGB Images
```lisp
( RGB    
    ( Sqrt ( Sin ( Abs Y ) ) )     
    ( Atan ( Atan2 ( + X ( / ( Ridge Y -0.30377412 Y ) -0.4523425 ) ) ( + ( Turbulence 0.95225644 ( Tan Y ) Y ) -0.46079302 ) ) )    
    ( Cell1 ( Ridge ( Ridge Y -0.83537865 -0.50440097 ) ( Atan2 Y X ) ( Sin 0.20003605 ) ) ( Sqrt ( Cell1 ( FBM Y X 0.8879242 ) 0.23509383 -0.4539826 ) ) ( Atan2 ( * X ( Ridge 0.6816149 X Y ) ) ( Cell1 ( Sin ( Turbulence X -0.25605845 Y ) ) -0.30595016 Y ) ) ) )
```

![Sample Image](/samples/rgb_noise.png)


### Gradient Images
```lisp
( Gradient
 ( Colors  ( 0.28973937 0.40621173 0.4788941 ) ( 0.88590646 0.9958223 0.6819649 ) ( 0.623574 0.39478934 0.97536874 ) ( 0.5160972 0.011721611 0.055956483 ) ( 0.88893497 0.8329935 0.587783 ) 
 ( Cell1 Y X -0.9553273 ) ) )
 ```

![Sample Image](/samples/gradient.png)
