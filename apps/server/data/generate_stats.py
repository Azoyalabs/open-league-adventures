
RAW = """
Stat	Warrior	Thief	Monk	W.Mage	B.Mage	R.Mage
HP	27	18	20	15	12	13
MP	18	24	0	45	49	27
Str	49	33	24	18	12	25
Agl	35	32	25	17	13	17
Int	14	17	24	33	48	24
Sta	25	16	49	19	14	21
Lck	24	49	30	20	15	25
"""

RAW_INCREASES = """Lv	Warrior	Thief	Monk	W.Mage	B.Mage	R.Mage
02	HP, Str	HP, Str, Lck	HP, Lck	HP, MP, Int	MP, Int	Str
03	Agl, Sta	Agl	Agl, Sta	Sta	-	MP, Agl
04	HP, Str, Lck	HP, Str, Int, Lck	Str, Int, Lck	Str, Lck	HP, MP, Int	Str, Sta, Lck
05	Agl, Sta	Sta	Sta	Agl	-	Int
06	HP, Str, Int, Lck	HP, Str, Lck	HP	HP, MP	MP, Agl, Int, Lck	HP, Str, Sta
07	Agl	Agl	Agl, Sta	Str, Sta	Sta	MP, Int
08	HP, Str	HP, Str, Int, Lck	Str, Int, Lck	HP, Int, Lck	HP, MP, Int	Str, Lck
09	Agl, Sta	-	Sta	-	-	MP, Agl
10	HP, Str, Lck	Str, Lck	Lck	MP	MP, Str, Int, Lck	-
11	Agl, Sta	Agl, Sta	Agl, Sta	Agl, Int, Sta	-	Int, Sta
12	HP, Str, Int, Lck	HP, Str, Int, Lck	HP, Str, Int	HP, Lck	MP, Agl, Int, Sta	HP, MP, Str,
Agl, Lck
13	Agl	-	Sta	MP, Str, Int	-	Sta
14	HP, Str	Str, Lck	Lck	HP	HP, MP, Int	MP, Str
15	Agl, Sta	Agl	Agl, Sta	MP, Sta	-	Int, Sta
16	HP, Str, Lck	HP, Str, Lck	Str, Int, Lck	Lck	MP, Int, Sta	HP, MP, Str, Lck
17	Agl, Sta	Sta	Sta	HP, MP, Agl	-	Sta
18	HP, Str, Int, Lck	Str, Int, Lck	HP	Int	HP, MP, Agl, Int, Lck  MP, Agl
19	Agl	Agl	Agl, Sta	MP, Str, Sta	Str	Int
20	HP, Str	HP, Lck	Str, Int, Lck	HP, Lck	MP, Int	HP, MP, Str, Lck
21	Agl, Sta	-	Sta	MP	-	Agl, Sta
22	Str, Lck	Str, Lck	Lck	HP, Int	MP, Int, Lck	Str
23	Agl, Sta	Agl, Sta	Agl, Sta	MP, Agl, Lck	-	MP, Int, Sta
24	HP, Str, Int, Lck	HP, Str, Int, Lck	HP, Str, Int	HP, Int, Sta	MP, Agl, Int, Sta	HP, Lck
25	Agl	-	Sta	MP, Str	Str	-
26	HP, Str	Lck	Lck	HP, Lck	HP, MP, Int	MP, Sta, Lck
27	Agl, Sta	Agl	Agl, Sta	MP, Sta	-	Agl, Int
28	Str, Lck	HP, Str, Lck	Str, Int, Lck	HP, Int	MP, Int	MP, Str
29	MP, Agl, Sta	MP, Sta	Sta	MP, Agl	-	-
30	HP, Str, Int, Lck	Str, Int, Lck	HP	-	MP, Agl, Int, Lck	MP, Int, Lck
31	Agl	Agl	Agl, Sta	MP, Str	Str	HP
32	HP, Str	Lck	HP, Str, Int, Lck	HP, Int, Lck	MP, Int, Sta	MP, Str, Sta, Lck
33	MP, Agl, Sta	MP, Agl	Sta	MP, Sta	-	Agl, Int
34	Str, Lck	HP, Str, Lck	Lck	Int	MP, Int, Lck	-
35	Agl, Sta	MP, Sta	Agl, Sta	MP, Agl	HP	MP, Sta, Lck
36	HP, Str, Int, Lck	Str, Int, Lck	HP, Str, Int	Sta	MP, Agl, Int, Sta	Str, Agl
37	MP, Agl	Agl	Sta	HP, MP, Str	Str	HP, Int
38	HP, Str	Lck	Lck	Int, Lck	MP, Int	Str, Sta, Lck
39	Agl, Sta	MP, Agl	Agl, Sta	MP	-	Int
40	Str, Lck	HP, Str, Lck	Str, Int, Lck	Int	MP, Int	MP
41	MP, Sta	MP, Sta	Sta	MP, Agl	HP	Sta, Lck
42	HP, Str, Int, Lck	Str, Int, Lck	HP	HP, Int, Sta	MP, Int, Lck	HP, Str, Agl
43	Agl	Agl	Agl, Sta	MP, Str, Int	-	Int
44	HP, Str	Lck	Str, Int, Lck	Lck	MP, Int, Sta	MP, Lck
45	MP, Agl, Sta	MP, Agl	Sta	MP, Int	Agl	Agl
46	Str, Lck	HP, Str, Lck	Lck	-	MP, Int	-
47	Sta	MP, Sta	Agl, Sta	HP, MP, Agl, Int	Str	HP, Int, Sta
48	Str, Int, Lck	Str, Int, Lck	HP, Str, Int	Sta	HP, MP, Int	Str, Agl, Lck
49	MP, Agl	Agl	Sta	MP, Str, Int	-	MP
50	HP, Str	Lck	Lck	Lck	MP, Int, Lck	Sta, Lck
51	Agl, Sta	MP, Agl	Agl, Sta	MP	Sta	Str
52	Str, Lck	HP, Str, Lck	Str, Int, Lck	Int	MP, Int	HP, MP
53	MP, Sta	MP, Sta	Sta	MP, Agl, Sta	-	Agl, Int
54	Str, Int, Lck	Str, Int, Lck	HP	-	HP, MP, Str, Int	Str, Sta
55	Agl	Agl	Agl, Sta	MP, Str, Lck	Agl	-
56	HP, Str	Lck	Str, Int, Lck	Int	MP, Int, Lck	MP, Lck
57	MP, Agl, Sta	MP, Agl	Sta	MP, Agl	-	-
58	Str, Lck	HP, Lck	Lck	-	MP, Int, Sta	Str, Agl, Int, Sta
59	Sta	MP, Sta	Agl, Sta	MP, Str, Sta	-	HP
60	Str, Int, Lck	Str, Int, Lck	HP, Str, Int	Lck	MP, Int	MP
61	MP, Agl	Agl	Sta	MP, Agl, Int	-	Lck
62	HP, Str	Lck	HP, Lck	-	HP, MP, Str, Int	Int
63	Sta	MP, Agl	Agl, Sta	Str, Sta	-	Sta
64	Str, Lck	HP, Str, Lck	Str, Int, Lck	MP	MP, Agl, Int, Lck	HP, MP, Str, Agl
65	MP, Agl	MP, Agl, Sta	Sta	Agl, Int, Lck	-	-
66	Str, Int	Str, Int, Lck	HP	MP	MP, Int	Int
67	Sta	Agl	Agl, Sta	Str, Int	-	Str, Lck
68	HP, Str, Lck	Lck	Str, Int, Lck	MP, Agl, Sta	MP, Int	-
69	MP, Agl	MP, Agl	Sta	Int	Agl	Int
70	Str	Str, Lck	Lck	MP, Str	MP, Int, Sta	-
71	Sta	MP, Agl, Sta	Agl, Sta	Int, Lck	Str	MP, Str
72	Str, Int, Lck	HP, Str, Int, Lck	HP, Str, Int	MP, Agl	MP, Int, Lck	Sta
73	MP, Agl	Agl	Sta	Int, Sta	-	HP, Agl, Lck
74	HP, Str	Lck	Lck	MP, Str	HP, MP, Int	-
75	Sta	MP, Agl	Agl, Sta	Lck	Agl	Int
76	Str, Lck	Str, Lck	Str, Int, Lck	MP, Int	MP, Int, Sta, Lck	-
77	MP, Agl	MP, Agl, Sta	Sta	Agl, Sta	-	MP, Str
78	Str, Int	Str, Int, Lck	HP	MP, Str	MP, Str, Int	Lck
79	Sta	Agl	Agl, Sta	Int	-	Int
80	HP, Str	HP, Lck	Str, Int, Lck	MP, Lck	MP, Int	-
81	MP, Agl	MP, Agl	Sta	-	Lck	Agl
82	Str	Str, Lck	Lck	MP, Str, Agl, Int	MP, Int, Sta	-
83	-	MP, Sta	Agl, Sta	-	-	MP, Str, Int, Lck
84	Str, Int	Int, Lck	HP, Str, Int	MP	MP, Int	-
85	MP, Agl	Agl	Sta	Int, Lck	-	-
86	HP, Str	Str, Lck	-	MP, Str	MP, Int	Lck
87	-	MP	Agl, Sta	Sta	Str, Agl	MP, Int
88	Str	Lck	Str, Int, Lck	MP, Int	MP, Int	Str
89	MP, Agl	MP, Agl, Sta	Sta	Agl	HP	-
90	Str	HP, Str, Int, Lck	HP	MP	MP, Lck	MP, Lck
91	-	-	Agl, Sta	Int, Lck	-	-
92	HP, Str	Lck	HP, Str, Int	MP	MP, Int, Sta	Int
93	MP, Agl	MP, Agl	Sta	-	-	Agl
94	Str	Str, Lck	Lck	MP, Int, Sta	MP, Int	Sta
95	-	MP, Sta	Agl, Sta	-	-	Str, Lck
96	Str	Int, Lck	HP, Str, Int	MP, Lck	MP, Agl, Int, Lck	MP
97	MP, Agl	Agl	Sta	-	Str	Int
98	HP, Str	Str, Lck	-	Int	MP, Int	Lck
99	-	MP	Agl, Sta	-	Sta	Sta
"""


BASE_STATS = {
    "Warrior": {"HP": 27, "MP": 18, "Str": 49, "Agl": 10, "Int": 14, "Sta": 25, "Lck": 24},
    "Thief": {"HP": 18, "MP": 24, "Str": 33, "Agl": 32, "Int": 17, "Sta": 16, "Lck": 49},
    "Monk": {"HP": 20, "MP": 0, "Str": 24, "Agl": 25, "Int": 24, "Sta": 49, "Lck": 30},
    "W.Mage": {"HP": 15, "MP": 45, "Str": 18, "Agl": 17, "Int": 33, "Sta": 19, "Lck": 20},
    "B.Mage": {"HP": 12, "MP": 49, "Str": 12, "Agl": 13, "Int": 48, "Sta": 14, "Lck": 15},
    "R.Mage": {"HP": 13, "MP": 27, "Str": 25, "Agl": 17, "Int": 24, "Sta": 21, "Lck": 25}
}

LEVEL_TABLE = {
    "2": {"Warrior": ["HP", "Str"], "Thief": ["HP", "Str", "Lck"], "Monk": ["HP", "Lck"], "W.Mage": ["HP", "MP", "Int"], "B.Mage": ["MP", "Int"], "R.Mage": ["Str"]},
    "3": {"Warrior": ["Agl", "Sta"], "Thief": ["Agl"], "Monk": ["Agl", "Sta"], "W.Mage": ["Sta"], "B.Mage": [], "R.Mage": ["MP", "Agl"]},
    "4": {"Warrior": ["HP", "Str", "Lck"], "Thief": ["HP", "Str", "Int", "Lck"], "Monk": ["Str", "Int", "Lck"], "W.Mage": ["Str", "Lck"], "B.Mage": ["HP", "MP", "Int"], "R.Mage": ["Str", "Sta", "Lck"]},
    "5": {"Warrior": ["Agl", "Sta"], "Thief": ["Sta"], "Monk": ["Sta"], "W.Mage": ["Agl"], "B.Mage": [], "R.Mage": ["Int"]},
    "6": {"Warrior": ["HP", "Str", "Int", "Lck"], "Thief": ["HP", "Str", "Lck"], "Monk": ["HP"], "W.Mage": ["HP", "MP"], "B.Mage": ["MP", "Agl", "Int", "Lck"], "R.Mage": ["HP", "Str", "Sta"]},
    "7": {"Warrior": [], "Thief": ["Agl"], "Monk": ["Agl", "Sta"], "W.Mage": ["Str", "Sta"], "B.Mage": ["Sta"], "R.Mage": ["MP", "Int"]},
    "8": {"Warrior": ["HP", "Str"], "Thief": ["HP", "Str", "Int", "Lck"], "Monk": ["Str", "Int", "Lck"], "W.Mage": ["HP", "Int", "Lck"], "B.Mage": ["HP", "MP", "Int"], "R.Mage": ["Str", "Lck"]},
    "9": {"Warrior": ["Agl", "Sta"], "Thief": [], "Monk": ["Sta"], "W.Mage": [], "B.Mage": [], "R.Mage": ["MP", "Agl"]},
    "10": {"Warrior": ["HP", "Str", "Lck"], "Thief": ["Str", "Lck"], "Monk": ["Lck"], "W.Mage": ["MP"], "B.Mage": ["MP", "Str", "Int", "Lck"], "R.Mage": []},
    "11": {"Warrior": ["Sta"], "Thief": ["Agl", "Sta"], "Monk": ["Agl", "Sta"], "W.Mage": ["Agl", "Int", "Sta"], "B.Mage": [], "R.Mage": ["Int", "Sta"]},
    "12": {"Warrior": ["HP", "Str", "Int", "Lck"], "Thief": ["HP", "Str", "Int", "Lck"], "Monk": ["HP", "Str", "Int"], "W.Mage": ["HP", "Lck"], "B.Mage": ["MP", "Agl", "Int", "Sta"], "R.Mage": ["HP", "MP", "Str", "Agl", "Lck"]},
    "13": {"Warrior": ["Agl"], "Thief": [], "Monk": ["Sta"], "W.Mage": ["MP", "Str", "Int"], "B.Mage": [], "R.Mage": ["Sta"]},
    "14": {"Warrior": ["HP", "Str"], "Thief": ["Str", "Lck"], "Monk": ["Lck"], "W.Mage": ["HP"], "B.Mage": ["HP", "MP", "Int"], "R.Mage": ["MP", "Str"]},
    "15": {"Warrior": ["Agl", "Sta"], "Thief": ["Agl"], "Monk": ["Agl", "Sta"], "W.Mage": ["MP", "Sta"], "B.Mage": [], "R.Mage": ["Int", "Sta"]},
    "16": {"Warrior": ["HP", "Str", "Lck"], "Thief": ["HP", "Str", "Lck"], "Monk": ["Str", "Int", "Lck"], "W.Mage": ["Lck"], "B.Mage": ["MP", "Int", "Sta"], "R.Mage": ["HP", "MP", "Str", "Lck"]},
    "17": {"Warrior": ["Agl", "Sta"], "Thief": ["Sta"], "Monk": ["Sta"], "W.Mage": ["HP", "MP", "Agl"], "B.Mage": [], "R.Mage": ["Sta"]},
    "18": {"Warrior": ["HP", "Str", "Int", "Lck"], "Thief": ["Str", "Int", "Lck"], "Monk": ["HP", "Str", "Int"], "W.Mage": ["HP", "Int"], "B.Mage": ["HP", "MP", "Agl", "Int", "Lck"], "R.Mage": ["MP", "Agl"]},
    "19": {"Warrior": [], "Thief": ["Agl"], "Monk": ["Agl", "Sta"], "W.Mage": ["MP", "Str", "Sta"], "B.Mage": ["Str"], "R.Mage": ["Int"]},
    "20": {"Warrior": ["HP", "Str"], "Thief": ["HP", "Lck"], "Monk": ["Str", "Int", "Lck"], "W.Mage": ["HP", "Lck"], "B.Mage": ["MP", "Int"], "R.Mage": ["HP", "MP", "Str", "Lck"]},
    "21": {"Warrior": ["Agl", "Sta"], "Thief": [], "Monk": ["Sta"], "W.Mage": ["MP"], "B.Mage": [], "R.Mage": ["Agl", "Sta"]},
    "22": {"Warrior": ["Str", "Lck"], "Thief": ["Str", "Lck"], "Monk": ["Lck"], "W.Mage": ["HP", "Int"], "B.Mage": ["MP", "Int", "Lck"], "R.Mage": ["Str"]},
    "23": {"Warrior": ["Sta"], "Thief": ["Agl", "Sta"], "Monk": ["Agl", "Sta"], "W.Mage": ["MP", "Agl", "Lck"], "B.Mage": [], "R.Mage": ["MP", "Int", "Sta"]},
    "24": {"Warrior": ["HP", "Str", "Int", "Lck"], "Thief": ["HP", "Str", "Int", "Lck"], "Monk": ["HP", "Str", "Int"], "W.Mage": ["HP", "Int", "Sta"], "B.Mage": ["MP", "Agl", "Int", "Sta"], "R.Mage": ["HP", "Lck"]},
    "25": {"Warrior": ["Agl"], "Thief": [], "Monk": ["Sta"], "W.Mage": ["MP", "Str"], "B.Mage": ["Str"], "R.Mage": []},
    "26": {"Warrior": ["HP", "Str"], "Thief": ["Lck"], "Monk": ["Lck"], "W.Mage": ["HP", "Lck"], "B.Mage": ["HP", "MP", "Int"], "R.Mage": ["MP", "Sta", "Lck"]},
    "27": {"Warrior": ["Agl", "Sta"], "Thief": ["Agl"], "Monk": ["Agl", "Sta"], "W.Mage": ["MP", "Sta"], "B.Mage": [], "R.Mage": ["Agl", "Int"]},
    "28": {"Warrior": ["Str", "Lck"], "Thief": ["HP", "Str", "Lck"], "Monk": ["Str", "Int", "Lck"], "W.Mage": ["HP", "Int"], "B.Mage": ["MP", "Int"], "R.Mage": ["MP", "Str"]},
    "29": {"Warrior": ["MP", "Agl", "Sta"], "Thief": ["MP", "Sta"], "Monk": ["Sta"], "W.Mage": ["MP", "Agl"], "B.Mage": [], "R.Mage": []},
    "30": {"Warrior": ["HP", "Str", "Int", "Lck"], "Thief": ["Str", "Int", "Lck"], "Monk": ["HP"], "W.Mage": [], "B.Mage": ["MP", "Agl", "Int", "Lck"], "R.Mage": ["MP", "Int", "Lck"]},
    "31": {"Warrior": ["Agl"], "Thief": ["Agl"], "Monk": ["Agl", "Sta"], "W.Mage": ["MP", "Str"], "B.Mage": ["Str"], "R.Mage": ["HP"]},
    "32": {"Warrior": ["HP", "Str"], "Thief": ["Lck"], "Monk": ["HP", "Str", "Int", "Lck"], "W.Mage": ["HP", "Int", "Lck"], "B.Mage": ["MP", "Int", "Sta"], "R.Mage": ["MP", "Str", "Sta", "Lck"]},
    "33": {"Warrior": ["MP", "Sta"], "Thief": ["MP", "Agl"], "Monk": ["Sta"], "W.Mage": ["MP", "Sta"], "B.Mage": [], "R.Mage": ["Agl", "Int"]},
    "34": {"Warrior": ["Str", "Lck"], "Thief": ["HP", "Str", "Lck"], "Monk": ["Lck"], "W.Mage": ["Int"], "B.Mage": ["MP", "Int", "Lck"], "R.Mage": []},
    "35": {"Warrior": ["Agl", "Sta"], "Thief": ["MP", "Sta"], "Monk": ["Agl", "Sta"], "W.Mage": ["MP", "Agl"], "B.Mage": ["HP"], "R.Mage": ["MP", "Sta", "Lck"]},
    "36": {"Warrior": ["HP", "Str", "Int", "Lck"], "Thief": ["Str", "Int", "Lck"], "Monk": ["HP", "Str", "Int"], "W.Mage": ["Sta"], "B.Mage": ["MP", "Agl", "Int", "Sta"], "R.Mage": ["Str", "Agl"]},
    "37": {"Warrior": ["MP", "Agl"], "Thief": ["Agl"], "Monk": ["Sta"], "W.Mage": ["HP", "MP", "Str"], "B.Mage": ["Str"], "R.Mage": ["HP", "Int"]},
    "38": {"Warrior": ["HP", "Str"], "Thief": ["Lck"], "Monk": ["Lck"], "W.Mage": ["Int", "Lck"], "B.Mage": ["MP", "Int"], "R.Mage": ["Str", "Sta", "Lck"]},
    "39": {"Warrior": ["Agl", "Sta"], "Thief": ["MP", "Agl"], "Monk": ["Agl", "Sta"], "W.Mage": ["MP"], "B.Mage": [], "R.Mage": ["Int"]},
    "40": {"Warrior": ["Str", "Lck"], "Thief": ["HP", "Str", "Lck"], "Monk": ["Str", "Int", "Lck"], "W.Mage": ["Int"], "B.Mage": ["MP", "Int"], "R.Mage": ["MP"]},
    "41": {"Warrior": ["MP", "Sta"], "Thief": ["MP", "Sta"], "Monk": ["Sta"], "W.Mage": ["MP", "Agl"], "B.Mage": ["HP"], "R.Mage": ["Sta", "Lck"]},
    "42": {"Warrior": ["HP", "Str", "Int", "Lck"], "Thief": ["Str", "Int", "Lck"], "Monk": ["HP"], "W.Mage": ["HP", "Int", "Sta"], "B.Mage": ["MP", "Int", "Lck"], "R.Mage": ["HP", "Str", "Agl"]},
    "43": {"Warrior": ["Agl"], "Thief": ["Agl"], "Monk": ["Agl", "Sta"], "W.Mage": ["MP", "Str", "Int"], "B.Mage": [], "R.Mage": ["Int"]},
    "44": {"Warrior": ["HP", "Str"], "Thief": ["Lck"], "Monk": ["Str", "Int", "Lck"], "W.Mage": ["Str", "Int", "Lck"], "B.Mage": ["MP", "Int", "Sta"], "R.Mage": ["MP", "Lck"]},
    "45": {"Warrior": ["MP", "Agl", "Sta"], "Thief": ["MP", "Agl"], "Monk": ["Sta"], "W.Mage": ["MP", "Int"], "B.Mage": ["Agl"], "R.Mage": ["Agl"]},
    "46": {"Warrior": ["Str", "Lck"], "Thief": ["HP", "Str", "Lck"], "Monk": ["Lck"], "W.Mage": [], "B.Mage": ["MP", "Int"], "R.Mage": []},
    "47": {"Warrior": ["Sta"], "Thief": ["MP", "Sta"], "Monk": ["Agl", "Sta"], "W.Mage": ["HP", "MP", "Agl", "Int"], "B.Mage": ["Str"], "R.Mage": ["HP", "Int", "Sta"]},
    "48": {"Warrior": ["Str", "Int", "Lck"], "Thief": ["Str", "Int", "Lck"], "Monk": ["HP", "Str", "Int"], "W.Mage": ["Sta"], "B.Mage": ["HP", "MP", "Int"], "R.Mage": ["Str", "Agl", "Lck"]},
    "49": {"Warrior": ["MP", "Agl"], "Thief": ["Agl"], "Monk": ["Sta"], "W.Mage": ["MP", "Str", "Int"], "B.Mage": [], "R.Mage": ["MP"]},
    "50": {"Warrior": ["HP", "Str"], "Thief": ["Lck"], "Monk": ["Lck"], "W.Mage": ["Lck"], "B.Mage": ["MP", "Int", "Lck"], "R.Mage": ["Sta", "Lck"]},
    "51": {"Warrior": ["Agl", "Sta"], "Thief": ["MP", "Agl"], "Monk": ["Agl", "Sta"], "W.Mage": ["MP"], "B.Mage": ["Sta"], "R.Mage": ["Str"]},
    "52": {"Warrior": ["Str", "Lck"], "Thief": ["HP", "Str", "Lck"], "Monk": ["Str", "Int", "Lck"], "W.Mage": ["Int"], "B.Mage": ["MP", "Int"], "R.Mage": ["HP", "MP"]},
    "53": {"Warrior": ["MP", "Sta"], "Thief": ["MP", "Sta"], "Monk": ["Sta"], "W.Mage": ["MP", "Agl", "Sta"], "B.Mage": [], "R.Mage": ["Agl", "Int"]},
    "54": {"Warrior": ["Str", "Int", "Lck"], "Thief": ["Str", "Int", "Lck"], "Monk": ["HP"], "W.Mage": [], "B.Mage": ["HP", "MP", "Str", "Int"], "R.Mage": ["Str", "Sta"]},
    "55": {"Warrior": ["Agl"], "Thief": ["Agl"], "Monk": ["Agl", "Sta"], "W.Mage": ["MP", "Str", "Lck"], "B.Mage": ["Agl"], "R.Mage": []},
    "56": {"Warrior": ["HP", "Str"], "Thief": ["Lck"], "Monk": ["Str", "Int", "Lck"], "W.Mage": ["Int"], "B.Mage": ["MP", "Int", "Lck"], "R.Mage": ["MP", "Lck"]},
    "57": {"Warrior": ["MP", "Agl", "Sta"], "Thief": ["MP", "Agl"], "Monk": ["Sta"], "W.Mage": ["MP", "Agl"], "B.Mage": [], "R.Mage": []},
    "58": {"Warrior": ["Str", "Lck"], "Thief": ["HP", "Lck"], "Monk": ["Lck"], "W.Mage": [], "B.Mage": ["MP", "Int", "Sta"], "R.Mage": ["Str", "Agl", "Int", "Sta"]},
    "59": {"Warrior": ["Sta"], "Thief": ["MP", "Sta"], "Monk": ["Agl", "Sta"], "W.Mage": ["MP", "Str", "Sta"], "B.Mage": [], "R.Mage": ["HP"]},
    "60": {"Warrior": ["Str", "Int", "Lck"], "Thief": ["Str", "Int", "Lck"], "Monk": ["HP", "Str", "Int"], "W.Mage": ["Lck"], "B.Mage": ["MP", "Int"], "R.Mage": ["MP"]},
    "61": {"Warrior": ["MP", "Agl"], "Thief": ["Agl"], "Monk": ["Sta"], "W.Mage": ["MP", "Agl", "Int"], "B.Mage": [], "R.Mage": ["Lck"]},
    "62": {"Warrior": ["HP", "Str"], "Thief": ["Lck"], "Monk": ["HP", "Lck"], "W.Mage": [], "B.Mage": ["HP", "MP", "Str", "Int"], "R.Mage": ["Int"]},
    "63": {"Warrior": ["Sta"], "Thief": ["MP", "Agl"], "Monk": ["Agl", "Sta"], "W.Mage": ["Str", "Sta"], "B.Mage": [], "R.Mage": ["Sta"]},
    "64": {"Warrior": ["Str", "Lck"], "Thief": ["HP", "Str", "Lck"], "Monk": ["Str", "Int", "Lck"], "W.Mage": ["MP"], "B.Mage": ["MP", "Agl", "Int", "Lck"], "R.Mage": ["HP", "MP", "Str", "Agl"]},
    "65": {"Warrior": ["MP", "Agl"], "Thief": ["MP", "Agl", "Sta"], "Monk": ["Sta"], "W.Mage": ["Agl", "Int", "Lck"], "B.Mage": [], "R.Mage": []},
    "66": {"Warrior": ["Str", "Int"], "Thief": ["Str", "Int", "Lck"], "Monk": ["HP"], "W.Mage": ["MP"], "B.Mage": ["MP", "Int"], "R.Mage": ["Int"]},
    "67": {"Warrior": ["Sta"], "Thief": ["Agl"], "Monk": ["Agl", "Sta"], "W.Mage": ["Str", "Int"], "B.Mage": [], "R.Mage": ["Str", "Lck"]},
    "68": {"Warrior": ["HP", "Str", "Lck"], "Thief": ["Lck"], "Monk": ["Str", "Int", "Lck"], "W.Mage": ["MP", "Agl", "Sta"], "B.Mage": ["MP", "Int"], "R.Mage": []},
    "69": {"Warrior": ["MP", "Agl"], "Thief": ["MP", "Agl"], "Monk": ["Sta"], "W.Mage": ["Int"], "B.Mage": ["Agl"], "R.Mage": ["Int"]},
    "70": {"Warrior": ["Str"], "Thief": ["Str", "Lck"], "Monk": ["Lck"], "W.Mage": ["MP", "Str"], "B.Mage": ["MP", "Int", "Sta"], "R.Mage": []},
    "71": {"Warrior": ["Sta"], "Thief": ["MP", "Agl", "Sta"], "Monk": ["Agl", "Sta"], "W.Mage": ["Int", "Lck"], "B.Mage": ["Str"], "R.Mage": ["MP", "Str"]},
    "72": {"Warrior": ["Str", "Int", "Lck"], "Thief": ["HP", "Str", "Int", "Lck"], "Monk": ["HP", "Str", "Int"], "W.Mage": ["MP", "Agl"], "B.Mage": ["MP", "Int", "Lck"], "R.Mage": ["Sta"]},
    "73": {"Warrior": ["MP", "Agl"], "Thief": ["Agl"], "Monk": ["Sta"], "W.Mage": ["Int", "Sta"], "B.Mage": [], "R.Mage": ["HP", "Agl", "Lck"]},
    "74": {"Warrior": ["HP", "Str"], "Thief": ["Lck"], "Monk": ["Lck"], "W.Mage": ["MP", "Str"], "B.Mage": ["HP", "MP", "Int"], "R.Mage": []},
    "75": {"Warrior": ["Sta"], "Thief": ["MP", "Agl"], "Monk": ["Agl", "Sta"], "W.Mage": ["Agl"], "B.Mage": ["Int"], "R.Mage": ["Int"]},
    "76": {"Warrior": ["Str", "Lck"], "Thief": ["Str", "Lck"], "Monk": ["Str", "Int", "Lck"], "W.Mage": ["MP", "Int"], "B.Mage": ["MP", "Int", "Sta", "Lck"], "R.Mage": []},
    "77": {"Warrior": ["MP", "Agl"], "Thief": ["MP", "Agl", "Sta"], "Monk": ["Sta"], "W.Mage": ["Agl", "Sta"], "B.Mage": [], "R.Mage": ["MP", "Str"]},
    "78": {"Warrior": ["Str", "Int"], "Thief": ["Str", "Int", "Lck"], "Monk": ["HP"], "W.Mage": ["MP", "Str"], "B.Mage": ["MP", "Str", "Int"], "R.Mage": ["Lck"]},
    "79": {"Warrior": ["Sta"], "Thief": ["Agl"], "Monk": ["Agl", "Sta"], "W.Mage": ["Int"], "B.Mage": [], "R.Mage": ["Int"]},
    "80": {"Warrior": ["HP", "Str"], "Thief": ["HP", "Lck"], "Monk": ["Str", "Int", "Lck"], "W.Mage": ["Str", "Int", "Lck"], "B.Mage": ["MP", "Int"], "R.Mage": []},
    "81": {"Warrior": ["MP", "Agl"], "Thief": ["MP", "Agl"], "Monk": ["Sta"], "W.Mage": ["Lck"], "B.Mage": ["Agl"], "R.Mage": []},
    "82": {"Warrior": ["Str"], "Thief": ["Str", "Lck"], "Monk": ["Lck"], "W.Mage": ["MP", "Str", "Agl", "Int"], "B.Mage": ["MP", "Int", "Sta"], "R.Mage": []},
    "83": {"Warrior": [], "Thief": ["MP", "Sta"], "Monk": ["Agl", "Sta"], "W.Mage": [], "B.Mage": [], "R.Mage": ["MP", "Str", "Int", "Lck"]},
    "84": {"Warrior": ["Str", "Int"], "Thief": ["Int", "Lck"], "Monk": ["HP", "Str", "Int"], "W.Mage": ["MP"], "B.Mage": ["MP", "Int"], "R.Mage": []},
    "85": {"Warrior": ["MP", "Agl"], "Thief": ["Agl"], "Monk": ["Sta"], "W.Mage": ["Int", "Lck"], "B.Mage": [], "R.Mage": []},
    "86": {"Warrior": ["HP", "Str"], "Thief": ["Str", "Lck"], "Monk": [], "W.Mage": ["MP", "Str"], "B.Mage": ["MP", "Int"], "R.Mage": ["Lck"]},
    "87": {"Warrior": [], "Thief": ["MP"], "Monk": ["Agl", "Sta"], "W.Mage": ["Sta"], "B.Mage": ["Str", "Agl"], "R.Mage": ["MP", "Int"]},
    "88": {"Warrior": ["Str"], "Thief": ["Lck"], "Monk": ["Str", "Int", "Lck"], "W.Mage": ["MP", "Int"], "B.Mage": ["MP", "Int"], "R.Mage": ["Str"]},
    "89": {"Warrior": ["MP", "Agl"], "Thief": ["MP", "Agl", "Sta"], "Monk": ["Sta"], "W.Mage": ["Agl"], "B.Mage": ["HP"], "R.Mage": []},
    "90": {"Warrior": ["Str"], "Thief": ["HP", "Str", "Int", "Lck"], "Monk": ["HP"], "W.Mage": ["MP"], "B.Mage": ["MP", "Lck"], "R.Mage": ["MP", "Lck"]},
    "91": {"Warrior": [], "Thief": [], "Monk": ["Agl", "Sta"], "W.Mage": ["Int", "Lck"], "B.Mage": [], "R.Mage": []},
    "92": {"Warrior": ["HP", "Str"], "Thief": ["Lck"], "Monk": ["HP", "Str", "Int"], "W.Mage": ["MP"], "B.Mage": ["MP", "Int", "Sta"], "R.Mage": ["Int"]},
    "93": {"Warrior": ["MP", "Agl"], "Thief": ["MP", "Agl"], "Monk": ["Sta"], "W.Mage": [], "B.Mage": [], "R.Mage": ["Agl"]},
    "94": {"Warrior": ["Str"], "Thief": ["Str", "Lck"], "Monk": ["Lck"], "W.Mage": ["MP", "Int", "Sta"], "B.Mage": ["MP", "Int"], "R.Mage": []},
    "95": {"Warrior": [], "Thief": ["MP", "Sta"], "Monk": ["Agl", "Sta"], "W.Mage": [], "B.Mage": [], "R.Mage": ["Str", "Lck"]},
    "96": {"Warrior": ["Str", "Int"], "Thief": ["Int", "Lck"], "Monk": ["HP", "Str", "Int"], "W.Mage": ["MP", "Lck"], "B.Mage": ["MP", "Agl", "Int", "Lck"], "R.Mage": ["MP"]},
    "97": {"Warrior": ["MP", "Agl"], "Thief": ["Agl"], "Monk": ["Sta"], "W.Mage": [], "B.Mage": ["Str"], "R.Mage": ["Int"]},
    "98": {"Warrior": ["HP", "Str"], "Thief": ["Str", "Lck"], "Monk": [], "W.Mage": ["Int"], "B.Mage": ["MP", "Int"], "R.Mage": ["Lck"]},
    "99": {"Warrior": [], "Thief": ["MP"], "Monk": ["Agl", "Sta"], "W.Mage": [], "B.Mage": ["Sta"], "R.Mage": ["Sta"]}
}


ALL_CLASSES = [
    "Warrior", "Thief", "Monk", "W.Mage", "B.Mage", "R.Mage"
]

ALL_STATS = [
    "HP", "MP", "Str", "Agl", "Int", "Sta", "Lck"
]

# only atk, defense and speed  
from copy import deepcopy
from random import random, randint, seed

seed(0)

def dump_stats(): 
    all_cl = {}
    for cl in ALL_CLASSES:
        per_level_stats = {}
        per_level_stats[1] = BASE_STATS[cl]
        curr_level = 2
        while curr_level < 100:
            curr_stats = deepcopy(per_level_stats[curr_level - 1])
            # get stats increase 
            guaranteed_stats_increase = LEVEL_TABLE[str(curr_level)][cl]
            not_guaranteed = []
            for s in ALL_STATS:
                if s not in guaranteed_stats_increase:
                    not_guaranteed.append(s)
            guaranteed_stats_increase = list(set(guaranteed_stats_increase))
            not_guaranteed = list(set(not_guaranteed))
            for stat_name in guaranteed_stats_increase:
                if stat_name != "HP":
                    curr_stats[stat_name] += 1
                else:
                    curr_stats["HP"] += randint(20, 25)
            
            for stat_name in not_guaranteed:
                if stat_name != "HP":
                    if random() < 0.25:
                        curr_stats[stat_name] += 1
                else:
                    curr_stats["HP"] += int(curr_stats["Sta"] / 4)
            
            # and commit 
            per_level_stats[curr_level] = curr_stats
            curr_level += 1     
        
        all_cl[cl] = per_level_stats       

    return all_cl

from typing import Dict

def to_eternal_quest_stats(raw_stats: Dict[str, Dict[int, Dict[str, int]]]):
    # need to have attack, defense, speed per class per level 
    processed = {}
    for curr_class in ALL_CLASSES:
        # Stat	Warrior	Thief	Monk	W.Mage	B.Mage	R.Mage
        new_class_name: str = ""
        match curr_class:
            case "Warrior":
                new_class_name = "warrior"
            case "Thief":
                new_class_name = "ranger"
            case "W.Mage":
                new_class_name = "white_mage"
            case "B.Mage":
                new_class_name = "black_mage"
            case _:
                continue
                
        #  "HP", "MP", "Str", "Agl", "Int", "Sta", "Lck"
        """
        curr_class_data = {}
        for curr_level in range(1, 100):
            curr_class_data[str(curr_level)] = {
                "attack": raw_stats[curr_class][curr_level]["Str"] if (curr_class in ["Warrior", "Thief"]) else raw_stats[curr_class][curr_level]["Int"],
                "defense": raw_stats[curr_class][curr_level]["Sta"],
                "speed": raw_stats[curr_class][curr_level]["Agl"],
                "hp": raw_stats[curr_class][curr_level]["HP"],
            }
        """
        curr_class_data = []
        for curr_level in range(1, 100):
            temp = {"level": curr_level, "stats": {
                "attack": raw_stats[curr_class][curr_level]["Str"] if (curr_class in ["Warrior", "Thief"]) else raw_stats[curr_class][curr_level]["Int"],
                "defense": raw_stats[curr_class][curr_level]["Sta"],
                "speed": raw_stats[curr_class][curr_level]["Agl"],
                "hp": raw_stats[curr_class][curr_level]["HP"],
            } }
            curr_class_data.append(temp)
            
        processed[new_class_name] = curr_class_data
    return processed


cls_stats = dump_stats()

import json 

with open("stats_table.json", "w+") as f:
    f.write(json.dumps(cls_stats))


processed_stats = to_eternal_quest_stats(cls_stats)

with open("processed_stats.json", "w+") as f:
    f.write(json.dumps(processed_stats))



"""
from matplotlib import pyplot as plt 
from mpl_toolkits.mplot3d import Axes3D

lvl = "1"

atks = [
    processed_stats[class_name][lvl]["attack"] for class_name in processed_stats
]

defs = [
    processed_stats[class_name][lvl]["defense"] for class_name in processed_stats
]

speeds = [
    processed_stats[class_name][lvl]["speed"] for class_name in processed_stats    
]

labels =  [
    class_name for class_name in processed_stats
]

# Create scatter plot
#plt.scatter(atks, defs)

fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')
ax.scatter(atks, defs, speeds)

for i, label in enumerate(labels):
    #plt.text(atks[i], defs[i], label, fontsize=12, ha='center', va='bottom')
    ax.text(atks[i], defs[i], speeds[i], label, fontsize=12)


# Add title and labels
ax.set_title('Atk v. Def v. Speed')
ax.set_xlabel('Attack')
ax.set_ylabel('Defense')
ax.set_zlabel('Speed')


for name in ["warrior", "ranger", "white_mage", "black_mage"]:
    temp =  processed_stats[name]["1"]
    total = temp["attack"] + temp["defense"] + temp["speed"]
    hp = temp["hp"]
    print("{} Stats: total {}, hp {}".format(name, total, hp))


# Show plot
plt.show()
"""
