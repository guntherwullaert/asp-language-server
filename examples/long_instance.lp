% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).


cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

test :- test(COMEAGAIN).

QUICK AS A FOX

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).


cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).


% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).


%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

ANOTHER QUICK BROWN FOXkjkl

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 


blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).


% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).


%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(1,1,1).optInClass(1,2,0).optInClass(1,3,0).optInClass(1,4,1).optInClass(1,5,0).
optInClass(2,1,1).optInClass(2,2,1).optInClass(2,3,1).optInClass(2,4,0).optInClass(2,5,0).
optInClass(3,1,1).optInClass(3,2,1).optInClass(3,3,0).optInClass(3,4,0).optInClass(3,5,1).
optInClass(4,1,0).optInClass(4,2,1).optInClass(4,3,1).optInClass(4,4,0).optInClass(4,5,0).
optInClass(5,1,0).optInClass(5,2,0).optInClass(5,3,0).optInClass(5,4,1).optInClass(5,5,0).
optInClass(6,1,0).optInClass(6,2,1).optInClass(6,3,0).optInClass(6,4,0).optInClass(6,5,0).
optInClass(7,1,0).optInClass(7,2,1).optInClass(7,3,1).optInClass(7,4,1).optInClass(7,5,0).
optInClass(8,1,0).optInClass(8,2,0).optInClass(8,3,1).optInClass(8,4,1).optInClass(8,5,0).
optInClass(9,1,1).optInClass(9,2,0).optInClass(9,3,1).optInClass(9,4,1).optInClass(9,5,0).
optInClass(10,1,0).optInClass(10,2,0).optInClass(10,3,1).optInClass(10,4,0).optInClass(10,5,0).
optInClass(11,1,1).optInClass(11,2,0).optInClass(11,3,1).optInClass(11,4,0).optInClass(11,5,0).
optInClass(12,1,1).optInClass(12,2,1).optInClass(12,3,1).optInClass(12,4,0).optInClass(12,5,1).
optInClass(13,1,0).optInClass(13,2,1).optInClass(13,3,0).optInClass(13,4,1).optInClass(13,5,0).
optInClass(14,1,1).optInClass(14,2,0).optInClass(14,3,0).optInClass(14,4,1).optInClass(14,5,1).
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).

% csp001: car sequencing
%  data file 

classes(1).
classes(2).
classes(3).
classes(4).
classes(5).
classes(6).
classes(7).
classes(8).
classes(9).
classes(10).
classes(11).
classes(12).
classes(13).
classes(14).
classes(15).
classes(16).
classes(17).
classes(18).
cars(1).
cars(2).
cars(3).
cars(4).
cars(5).
cars(6).
cars(7).
cars(8).
cars(9).
cars(10).
cars(11).
cars(12).
cars(13).
cars(14).
cars(15).
cars(16).
cars(17).
cars(18).
cars(19).
cars(20).
cars(21).
cars(22).
cars(23).
cars(24).
cars(25).
cars(26).
cars(27).
cars(28).
cars(29).
cars(30).
cars(31).
cars(32).
cars(33).
cars(34).
cars(35).
cars(36).
cars(37).
cars(38).
cars(39).
cars(40).
cars(41).
cars(42).
cars(43).
cars(44).
cars(45).
cars(46).
cars(47).
cars(48).
cars(49).
cars(50).
cars(51).
cars(52).
cars(53).
cars(54).
cars(55).
cars(56).
cars(57).
cars(58).
cars(59).
cars(60).
cars(61).
cars(62).
cars(63).
cars(64).
cars(65).
cars(66).
cars(67).
cars(68).
cars(69).
cars(70).
cars(71).
cars(72).
cars(73).
cars(74).
cars(75).
cars(76).
cars(77).
cars(78).
cars(79).
cars(80).
cars(81).
cars(82).
cars(83).
cars(84).
cars(85).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).
cars(86).
cars(87).
cars(88).
options(1).
options(2).
options(3).
options(4).
options(5).

% max number of cars with i-th option per block
maxCarsInBlock(1,1).maxCarsInBlock(2,2).
maxCarsInBlock(3,1).maxCarsInBlock(4,2).
maxCarsInBlock(5,1).

% block size per option
blockSize(1,2).blockSize(2,3).blockSize(3,3).
blockSize(4,5).blockSize(5,5).

%  number of cars to be produced for each class
carPerClass(1,6).carPerClass(2,10).carPerClass(3,2).
carPerClass(4,2).carPerClass(5,8).carPerClass(6,15).
carPerClass(7,1).carPerClass(8,5).carPerClass(9,2).
carPerClass(10,3).carPerClass(11,2).carPerClass(12,1).
carPerClass(13,8).carPerClass(14,3).carPerClass(15,10).
carPerClass(16,4).carPerClass(17,4).carPerClass(18,2).

test :- test(WITH_VAR).


% option included in class
% optInClass(C,Opt,1) means that for class C option Opt  is request, 
% not otherwise
optInClass(15,1,1).optInClass(15,2,0).optInClass(15,3,0).optInClass(15,4,0).optInClass(15,5,0).
optInClass(16,1,0).optInClass(16,2,1).optInClass(16,3,0).optInClass(16,4,0).optInClass(16,5,1).
optInClass(17,1,0).optInClass(17,2,0).optInClass(17,3,0).optInClass(17,4,0).optInClass(17,5,1).
optInClass(18,1,1).optInClass(18,2,0).optInClass(18,3,0).optInClass(18,4,0).optInClass(18,5,1).