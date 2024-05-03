# BOM line condenser

Problem: When ordering electronics components from mouser you will end up with a lot of small plastic bags, each with a label that simply has the name, quantity and some details about the given component. However, when we want to solder a project with a BOM and instructions, then we refer to components with IDs such as for example C3 for capacitor number 3. All we care about while assembling and soldering is the IDs and not so much all the other details. Naturally mouser cannot know the BOM ids for what you are creating, so you have to fill in the ids yourself when ordering. A challenge when entering the ids, is that the text field in which you can enter the ids is very limited in length, so we need to shorten the id list as much as possible.

Solution: Condense a line in a BOM in order to create a short text label to use in mouser which eventually will end up as a label on the plastic bag that contains the ordered components. This makes the soldering process much easier.

for example
```
C3, C4, C5, C6, C9, C11, C13, C18
```
to
```
C:3-6,9,11,13,18
```
which can be inserted into the text field `Customer No` for the relevant component on mouser.

## pro-tips
### Prefix when ordering for multiple different PCBs
prefix `g` is short for Mutable Instruments Grids.
For example
```
gC:3-6,9,11,13,18
```
now it is easier to find the component plastic bags that are related to the MI Grids assembly.

### Uploading and creating 'projects' on mouser
Mouser has some excelent in-browser-tooling for uploading CSV BOM files and creating projects from them so you can organize your order. https://www.mouser.dk/bom/

## Alternative idea to id-condensing
An alternative could be that we generate a random ID or a hash of the BOM line, and then we insert this ID in the mouser component order and in the BOM excel sheet... However, it is much nicer if we can simply read the ids directly from the component plastic bag label.
