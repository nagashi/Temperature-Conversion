### <div align="center">Temperature Conversion

<div align="center">
    <img src="images/celcius-to-fahrenheit.gif" alt="Celcius to Fahrenheit" width="200" height="150" float="left" border="3" />
    &nbsp;&nbsp;
    <img src="images/fahrenheit-to-celsius.gif" alt="Fahrenheit to Celsius" width="200" height="150" float="left" border="3"/>
</div>
<br><br>

The Temperature Conversion Application is a simple application comprised of loop labels,&nbsp;enums,&nbsp;functions,&nbsp;a closure,&nbsp;and implementations of traits.&nbsp;&nbsp;Once the program is started,&nbsp;the user is asked to enter a 'C' for conversion from Celsius to Fahrenheit or 'F' to convert from Fahrenheit to Celsius,&nbsp;even though,&nbsp;the program is additionally designed to allow a user to input 'Celsius' or 'Fahrenheit'.&nbsp;&nbsp;The program then asks that a number be entered for the temperature to be converted.&nbsp;&nbsp;The number can either be of type integer or float.&nbsp;&nbsp;At each stage of the previous steps, the user is given the option to exit the program by typing 'quit'.&nbsp;&nbsp;Any error in entering the requested type for either step will simply rerun that particular step.&nbsp;&nbsp;The flowchart for the program can be viewed below.

```mermaid
flowchart TB 
A(Start<br>Program);  %% round edges
B{Quit or enter<br>temperature type<br>C or F<br>for conversion};
C{Quit or enter<br>temperature number<br>integer or float};
D(Temperature<br>successfully converted!);  %% round edges
E(Stop<br>Program); %% round edges

%% Assign links
A --> B;
B -- C or F --> C;
B == Error on Entry ==> B;
C -- Integer or Float --> D;
C == Error on Entry ==> C; %% Bold edge
D -- Start Again -->B;
B -. Quit .-> E; %% dashes
C -. Quit .-> E;

%% Assign Style
style A stroke:#66ff00, stroke-width:5px;
style D stroke:#66ff00,stroke-width:5; 
style E stroke:red, stroke-width:5px;

%% Assign appropriate colors to specific links
linkStyle 2,4 stroke-width:5px,stroke:yellow,fill:none;
linkStyle 0,1,3 stroke-width:5px,stroke:#66ff00,fill:none;
linkStyle 5 stroke-width:5px,stroke:#66ff00,fill:none;
linkStyle 6,7 stroke-width:5px,fill:none,stroke:red;
%% End block of code
```
Thank you for reading about this application.&nbsp;&nbsp;Click 'Star' if you like the program and do reach out and let me know if you have any questions or concerns.&nbsp;&nbsp;All suggestions, constructive,&nbsp;even non-constructive,&nbsp;will be welcomed.<img src = "images/ok.png" alt="Image denoting Ok"
          width="30"
          height="20"
          border="0"
        />&nbsp;The source code can be viewed by first clicking on the&nbsp; [src](https://github.com/nagashi/Temperature-Conversion/tree/main/src)&nbsp;folder,&nbsp;and then clicking on the&nbsp; [main.rs](https://github.com/nagashi/Temperature-Conversion/blob/main/src/main.rs)&nbsp;file,&nbsp;for those unfamiliar with the Rust language.        
        <br>
<div align="center"> 

[![MIT licensed][mit-badge]][mit-url]&nbsp;&nbsp;[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)<br>Rust 🦀 v1.59.0

</div>
<br>
<br>
<p>

### License

This project is licensed under the&nbsp; [MIT license](LICENSE).

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE
### Contribution

Unless you explicitly state otherwise,&nbsp;any contribution intentionally submitted
for inclusion in&nbsp; 'Temperature_Conversion'&nbsp;by you,&nbsp;shall be licensed as MIT,&nbsp;without any additional terms or conditions.

</p>