Main program is reta or reta.py.
More convenient is retaPrompt, which is still available with presets as rp and rpl.

User manual:
There are 4 main parameters.
**Important: the secondary parameters must be placed directly after the correct main parameter, otherwise they have no effect and no other main parameter may be placed in between!**
Main parameters start with a minus -.
Secondary parameters start with 2 minus --.

# main parameter

## -debug
    * has no secondary parameters, is only relevant and interesting for me as a programmer

## -lines

    * --all
    * --time=
        * "yesterday"
            means religions 1-9
        * "today"
            means only religion 10
        * "tomorrow"
            means religions > 10
        * "yesterday,today,tomorrow"
            means religion 1-10 and higher than 10,
        * "-yesterday,-today,-tomorrow"
            to substract
    * --counting=
        * 1,2,3,4,5,...
    * -type=
        * sun,moon,planet,black_sun,sunWithMoonParts
        * -sun,-moon,-planet,-black_sun,-sunWithMoonParts
    * --primenumbers=
        * insidefirst,insideall,outsidefirst,outsideall
        * -insidefirst,-insideall,-outsidefirst,-outsideall
    * --multiplesofnumbers=
        * 1,2,3,4,5,...
    * --primemultiples=
        * 1,2,3,4,5,...
    * --thisrangebefore=
        * 1-5,7-10,14,20
    * --thisrangebeforedividers
        * causes that the divisors of all numbers, which result from the specification of "--beforefromsection=", are added additionally
        * e.g. 12 becomes: 2,3,4,6,12
    * --retrospectiverecount=
        * 3-6,8
        * For this the result rows are recounted. If lines "5 to 7" were previously determined and line 2 is now selected with this, it would be line 6.
    * ---retrospectiverecountmultiples=
        * 3-6,8
        * For this the result lines are counted again. If lines "5 to 8" were determined before and now line 2 is chosen with this, this would be line 6.8, because recounting lines "5 to 8" results in lines "1 to 4". Of these, every second line is 2 and 4. Calculated back to lines "5 to 8", these are lines 6 and 8.
    * --potenciesofnumbers=
        * 2,3
    * --uppermaximum
        * 2000,1500
    * --invert
        * chooses neighbors

## --columns

    * --all
    * --most_important_to_understand=
    * --most_important_to_classify
        * most important,second most important
    * --width=
        * 40
        * 70
    * --widths=
        * 20,50,10,70
    * --mostimportanttounderstand= 
        * mostimportant,secondmostimportant,thirdmostimportant,fourthmostimportant
    * --human= 
        * human-animal,views,directions,class,morality,specialties,senseandmeaningoflife,intelligenceproblems,way_of_thinking_of_living_beings,countertranscendentals,equalityfreedom,emotions,egoism,effect,incel,irrationalNumbersbyrooting,dominantgender,love,faith,vulnerability,motifs,positions,consciousness,achievements,evolutionary_acquire_and_intelligence,need,disease,alphabeta,lead,manipulation,jobs,solutions,music,honest
    * --planet= 
        * truthRealities,metasystems,intelligence,equality,complexity,mechanisms
    * --mostImportantToClassify= 
        * mostimportant,secondmostimportant
    * --operations= 
        * halving,five,nine,three,two,multiplication,four
    * --religions= 
        * religionfoundertype,hinduism,starpolygon,ofthezodiac,compare,messiah,uniformpolygon,representativehigherconcepts
    * --galaxy= 
        * revelation,breedingup,spherescircles,chinesehoroscope,zodiacsigns,gospelofthomas,analyticontology,insideOutsideStructure,modallogic,space
    * --increment= 
        * addition,of1,of2,of3,whytranscendentalstructuralsizeandcharacter,whyTranscendentalEqualsComplexity
    * --universe= 
        * artificiallife,strategy,ratioSameNumber,law,combineetc,breedingup,particle-meta-physics,metaparadigmreligion,spherescircles,analyticontology,countertranscendentals,systemthings,transcendentals,transcendentalreciprocal,network,whytranscendentalstructuralsizeandcharacter,category,space,programmingparadigms,mind,whyTranscendentalEqualsComplexity,modelofhierarchicalcomplexity
    * --particles= 
        * universal_(16),truthRealities,galactic_(14),multiverse_(16),non_value_virtue_sorting_(13_with_14),galaxy_sub_areas_(13),goodness_and_direction_(7),space_and_dimension_(8)
    * --multiverse= 
        * multiversalias,particles
    * --basic_structures= 
        * network,mathematicaldesign,analyticontology,modelofhierarchicalcomplexity,transcendentals,condition,areas_of_life,measures,relativereciprocal,universalcomparative,existentials,extremalia,expectations,passion,relativetimeamount,numericalcomparison,aspiration,principles,attractions,optimization,topics,meaning,reciprocal,attention,time,intention16,intention17,intention6,intention7,awareness,behavior,energy,sheafs,comprehend,empathy,intention1/6,innervalues,intention10,mind,reflex,lust,paradigms,truthRealities,calculate,mood,class,order,metasystems,intention1pro8,goals,concreta,emotions,dependence,map,foundation,positions,imaginations,shall,views,affiliations,intention13,love,coalitions,against_5,impulses,drive,reflection,states
    * --scale= 
        * light,scale,organizations,politicalsystems
    * --universeMetaConcrete= 
        * meta,concrete,theory,practice,management,changing,mathematical_discrete,beyond,enterprise,value,rule,direction
    * --primeeffect= 
        * universe,directiondirection,intentionGalaxy,intentGalaxyReciprocal,universeReciprocal,against-against-transcendentalism,neutral_against_transcendentalism
    * --economy= 
        * specialties,plants,machines,typeoforganization,system,explanation,businessadministration
    * --procontra= 
        * resultsSense,change,tamecontrol,unity,advantages,opponent,annoying,probenefit,counterposition,helpreceive,help,pro,notGettingAlong,notAgainst,noopposite,notfor,helpnotneed,cannothelp,notdisposed,unmotivated,against,opposite,harmony,primecross
    * --light 
    * --meaning= 
        * primecross,inreta,signbefore,primenumbers,applicationofsunsandmoons,ranges,law,perfection,spaceObject,subjunctive,mechanisms
    * --gebrochengalaxie_fractionalgalaxy= 
        * 18,15,13,21,3,9,19,2,5,12,8,20,11,22,6,4,16,17,23,7,10,14
    * --gebrochenuniversum_fractionaluniverse= 
        * 18,15,13,21,3,9,19,2,5,12,8,20,11,22,6,4,16,17,23,7,10,14
    * --fractionedemotion= 
        * 18,15,13,21,3,9,19,2,5,12,8,20,11,22,6,4,16,17,23,7,10,14
    * --fractionalnumberedstructuresize= 
        * 18,15,13,21,3,9,19,2,5,12,8,20,11,22,6,4,16,17,23,7,10,14
    * --symbols 
    * --properties= 
        * wisdom,rights,inferior,dispute,latchOn,familyneeds,humble,selfishness,science,asshole,love,selfless,monotonous,averse,honest,scope,worthless,familial,soft,uniteConnect,similar,good,senseAndMeaning,time,egalitarianAuthoritarian,opinions,opinionintelligence,morality,leadership,X-ray,encourage,arrogant,polarityOfLove,egoism,cachetPrestigeValidity,equal,survive
    * --concept2= 
        * worthy,rule,filterType,values,position,reflect,wantToTrust,align,tolerance
    * --multiplications= 
        * frame,motifRegularPolygons,structureRegularPolygons,motifStar,structureStar,motifStar,strukgebrstern,motifRegularf,structureRegularF,described


## -combination
    * --galaxy=
        * livingbeing,animals,animal,creatures,jobs,jobs,job,creativity_and_intelligence,creativity,intelligence,creativity,love,love,men,men,men,women,personality_evolutionary_acquisition,evolution,acquire,personality,personality,religion,religion,religions,motives_goals,motivation,goals,goal,motifs,Emotions,emotions,emotions,emotions,emotion,feelings,people,persons,celebrities,celebrities,economicsystems,economicsystem,economicsystems,combinedeconomicsystem,combinedeconomicsystems,ownership_and_possession
    * --universe=
        * livingbeing,animals,animal,creatures,jobs,jobs,job,transcendentalia_structuralia,transcendence,transcendentals,structurals,philoalias,primecross,leibnitz,primecross,personality_evolutionary_acquisition,evolution,acquire,personality,personality,religion,religion,religions,motives_goals,motivation,motifs,goals,goal,analytic_ontology,analyticontology,ontology,people,persons,celebrities,celebrities,mechanisms_of_breeding,mechanisms,essence,breed,breed,countertranscendentals,countertranscendentals,counter-structuralias,machines,machines,devices,equipment,mind,mind,consciousness,consciousness


## -output
    * --nocolor
    * --kind=
        * (only one allowed)
        * shell,html,csv,markdown,bbcode
    * --onetable
    * --columnorderandonlythese=
        * 3,5,1
        * i.e. from e.g. 5 columns the 3rd, then 5th and 1st should be displayed first and the others not!
    * --no_blank_contents
        * This makes that rows are not output, which contain only a minus or question mark or otherwise almost no information.
    * --noheadings
        * Headings are not output.
    * --no_numbering
        * Number of line and number of numbering several lines do not become displayed.



## ranges
    * instead of 2-11
      * -2-11
    * instead of 7
      * -7
    * instead of --symbols
      * --symbols-
    * instead of --religions=star-polygon
      * --religions=star-polygon

## The plus synax: meant are neighbors
    * 7+1
      * results in
      * 6 and 8
      * This means that both neighbors of the 7 are used
      * With multiples these would be then always the neighbors of the 7 thus
      * 6,8,13,15,20,22, etc.
    * 9-11+3
      * In the range 9 to 11 the third neighbor is used, so:
      * 9 to 11 would first be the numbers 9,10,11.
      * Because it is not said
      * 9-11+0
      * it is not these numbers 9,10,11, but
      * instead of 9 it is the 6 and 12, because of the +3 in the syntax at 9-11+3
      * instead of 10 it is 7 and 13
      * instead of 11 it is 8 and 14

      * For multiples, the multiples of 9,10,11 are formed and then the neighbors of the distance +3 are used by subtraction and addition

    * 10+0+2+5
      * 10,12,8,5,15
      * The 10 with distance zero is the 10 itself.
      * Distance 2 to the 10 is 8 and 12
      * distance 5 to the 10 is 5 and 15
    * m5
      * In (almost) all such number specifications, a m can be written in front of it: This leads to multiples being used instead of just the number: in the m5 example, this means that instead of the number 5, it is now the numbers 5,10,15,20,25, etc.

    * m syntax thereby
      * 5,m20-22 means line 5 and also all multiples of 20,21,22, e.g. 40,42,44
      -20,m10 means all multiples of 10 without the 20 in it

### Example (one line, not several):
        `reta -lines --thisrangebefore=1-9 -columns --religions=starpolygon,uniformpolygon --galaxy=babylon --width=50`
    * Python Ranges are possible for ranges
        * in retaPrompt:
        `reta -lines --thisrangebefore={2*n for n in range(2,5)},10 -columns --human=motifs -output --columnorderandonlythese=[3*n for n in range(2)]`
        * in a Shell as Bash:
            `reta -lines "--thisrangebefore={2*n for n in range(2,5)},10" -columns --human=motifs -output "--columnorderandonlythese=[3*n for n in range(2)]"  -language=english`
        a minus before subtracts instead of adding ranges: -[n for n in range(3)]
        * instead generator {2*n for n in range(2,5)} python calculations are possible as [2*3].

Better read this with a markdown reader!
        
