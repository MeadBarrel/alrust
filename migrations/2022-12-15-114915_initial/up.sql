CREATE TABLE IF NOT EXISTS "player_characters" (
	"name"	TEXT NOT NULL,
	"advanced_potion_making"	INTEGER NOT NULL DEFAULT 0,
	"alvarin_clade"	BOOLEAN NOT NULL DEFAULT 0,
	PRIMARY KEY("name")
);
CREATE TABLE IF NOT EXISTS "player_character_lores" (
	"character"	TEXT NOT NULL,
	"lore"	TEXT NOT NULL,
	"value"	INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY("character","lore"),
	FOREIGN KEY("lore") REFERENCES "lores",
	FOREIGN KEY("character") REFERENCES "player_characters"
);
CREATE TABLE IF NOT EXISTS "lores" (
	"name"	TEXT NOT NULL,
	"effectiveness"	DECIMAL(12, 6),
	"parent"	TEXT,
	"parent2"	TEXT,
	PRIMARY KEY("name"),
	FOREIGN KEY("parent2") REFERENCES "lores"("name"),
	FOREIGN KEY("parent") REFERENCES "lores"("name")
);
CREATE TABLE IF NOT EXISTS "ingredients" (
	"name"	TEXT NOT NULL,
	"lore"	TEXT NOT NULL,
	"al_weight"	INTEGER NOT NULL,
	"dh"	DECIMAL(12, 2),
	"dp"	DECIMAL(12, 2),
	"mdh"	DECIMAL(12, 2),
	"mdp"	DECIMAL(12, 2),
	"hot"	DECIMAL(12, 2),
	"pot"	DECIMAL(12, 2),
	"mhot"	DECIMAL(12, 2),
	"mpot"	DECIMAL(12, 2),
	"hl"	DECIMAL(12, 2),
	"pl"	DECIMAL(12, 2),
	"mhl"	DECIMAL(12, 2),
	"mpl"	DECIMAL(12, 2),
	"a"	DECIMAL(12, 2),
	"ma"	DECIMAL(12, 2),
	"notes"	TEXT,
	PRIMARY KEY("name"),
	FOREIGN KEY("lore") REFERENCES "lores"("name")
);
INSERT INTO "lores" VALUES ('Alchemy',NULL,NULL,NULL);
INSERT INTO "lores" VALUES ('Zoology',NULL,NULL,NULL);
INSERT INTO "lores" VALUES ('Material Lore',NULL,NULL,NULL);
INSERT INTO "lores" VALUES ('Petrology',NULL,'Material Lore',NULL);
INSERT INTO "lores" VALUES ('Animal Materials',NULL,'Material Lore',NULL);
INSERT INTO "lores" VALUES ('Alchemy Contraptions',NULL,'Alchemy',NULL);
INSERT INTO "lores" VALUES ('Potion Making',NULL,'Alchemy',NULL);
INSERT INTO "lores" VALUES ('Fabricula Expertise',NULL,'Alchemy Contraptions',NULL);
INSERT INTO "lores" VALUES ('Advanced Potion Making',NULL,'Potion Making',NULL);
INSERT INTO "lores" VALUES ('Alchemical Dissolvents',NULL,'Alchemy','Petrology');
INSERT INTO "lores" VALUES ('Dragon Salt Lore',NULL,'Alchemical Dissolvents',NULL);
INSERT INTO "lores" VALUES ('Rock Oil Lore',NULL,'Alchemical Dissolvents',NULL);
INSERT INTO "lores" VALUES ('Fuming Salt Lore',NULL,'Alchemical Dissolvents',NULL);
INSERT INTO "lores" VALUES ('Nitre Lore',NULL,'Alchemical Dissolvents',NULL);
INSERT INTO "lores" VALUES ('Salt Lore',NULL,'Alchemical Dissolvents',NULL);
INSERT INTO "lores" VALUES ('Bore Lore',NULL,'Alchemical Dissolvents',NULL);
INSERT INTO "lores" VALUES ('Sulfur Lore',NULL,'Alchemical Dissolvents',NULL);
INSERT INTO "lores" VALUES ('Arthropoda',NULL,'Zoology',NULL);
INSERT INTO "lores" VALUES ('Amphibia',NULL,'Zoology',NULL);
INSERT INTO "lores" VALUES ('Pisces',NULL,'Zoology',NULL);
INSERT INTO "lores" VALUES ('Holozoa',NULL,'Zoology',NULL);
INSERT INTO "lores" VALUES ('Mammalia',NULL,'Zoology',NULL);
INSERT INTO "lores" VALUES ('Reptilia',NULL,'Zoology',NULL);
INSERT INTO "lores" VALUES ('Dragon Lore',NULL,'Zoology',NULL);
INSERT INTO "lores" VALUES ('Aves',NULL,'Zoology',NULL);
INSERT INTO "lores" VALUES ('Walker Knowledge',NULL,'Zoology',NULL);
INSERT INTO "lores" VALUES ('Trilobitomorpha',NULL,'Arthropoda',NULL);
INSERT INTO "lores" VALUES ('Chelicerata',NULL,'Arthropoda',NULL);
INSERT INTO "lores" VALUES ('Insecta',NULL,'Arthropoda',NULL);
INSERT INTO "lores" VALUES ('Myriapoda',NULL,'Arthropoda',NULL);
INSERT INTO "lores" VALUES ('Crustacea',NULL,'Arthropoda',NULL);
INSERT INTO "lores" VALUES ('Arachnida',NULL,'Chelicerata',NULL);
INSERT INTO "lores" VALUES ('Eurypterida',NULL,'Chelicerata',NULL);
INSERT INTO "lores" VALUES ('Xiphosura',NULL,'Chelicerata',NULL);
INSERT INTO "lores" VALUES ('Pycnogonida',NULL,'Chelicerata',NULL);
INSERT INTO "lores" VALUES ('Opiliones',NULL,'Arachnida',NULL);
INSERT INTO "lores" VALUES ('Araneae',NULL,'Arachnida',NULL);
INSERT INTO "lores" VALUES ('Scorpiones',NULL,'Arachnida',NULL);
INSERT INTO "lores" VALUES ('Acarina',NULL,'Arachnida',NULL);
INSERT INTO "lores" VALUES ('Anura',NULL,'Amphibia',NULL);
INSERT INTO "lores" VALUES ('Gymnophiona',NULL,'Amphibia',NULL);
INSERT INTO "lores" VALUES ('Caudata',NULL,'Amphibia',NULL);
INSERT INTO "lores" VALUES ('Cephalochordata',NULL,'Pisces',NULL);
INSERT INTO "lores" VALUES ('Tunicata',NULL,'Pisces',NULL);
INSERT INTO "lores" VALUES ('Placodermi',NULL,'Pisces',NULL);
INSERT INTO "lores" VALUES ('Agnatha',NULL,'Pisces',NULL);
INSERT INTO "lores" VALUES ('Chondrichtyes',NULL,'Pisces',NULL);
INSERT INTO "lores" VALUES ('Osteichthyes',NULL,'Pisces',NULL);
INSERT INTO "lores" VALUES ('Chondrostei',NULL,'Osteichthyes',NULL);
INSERT INTO "lores" VALUES ('Perciformes',NULL,'Osteichthyes',NULL);
INSERT INTO "lores" VALUES ('Acanthopterygii',NULL,'Osteichthyes',NULL);
INSERT INTO "lores" VALUES ('Holostei',NULL,'Osteichthyes',NULL);
INSERT INTO "lores" VALUES ('Teleostei',NULL,'Osteichthyes',NULL);
INSERT INTO "lores" VALUES ('Parazoa',NULL,'Holozoa',NULL);
INSERT INTO "lores" VALUES ('Radiata',NULL,'Holozoa',NULL);
INSERT INTO "lores" VALUES ('Bilateria',NULL,'Holozoa',NULL);
INSERT INTO "lores" VALUES ('Deuterostomia',NULL,'Bilateria',NULL);
INSERT INTO "lores" VALUES ('Brachiopoda',NULL,'Bilateria',NULL);
INSERT INTO "lores" VALUES ('Ecdysozoa',NULL,'Bilateria',NULL);
INSERT INTO "lores" VALUES ('Mollusca',NULL,'Bilateria',NULL);
INSERT INTO "lores" VALUES ('Annelida',NULL,'Bilateria',NULL);
INSERT INTO "lores" VALUES ('Pegasoferae',NULL,'Mammalia',NULL);
INSERT INTO "lores" VALUES ('Rodentia',NULL,'Mammalia',NULL);
INSERT INTO "lores" VALUES ('Evarchonta',NULL,'Mammalia',NULL);
INSERT INTO "lores" VALUES ('Insectivora',NULL,'Mammalia',NULL);
INSERT INTO "lores" VALUES ('Lagomorpha',NULL,'Mammalia',NULL);
INSERT INTO "lores" VALUES ('Xenarthra',NULL,'Mammalia',NULL);
INSERT INTO "lores" VALUES ('Paenungulata',NULL,'Mammalia',NULL);
INSERT INTO "lores" VALUES ('Cetartiodactyla',NULL,'Mammalia',NULL);
INSERT INTO "lores" VALUES ('Feliformia',NULL,'Pegasoferae',NULL);
INSERT INTO "lores" VALUES ('Chiroptera',NULL,'Pegasoferae',NULL);
INSERT INTO "lores" VALUES ('Perissodactyla',NULL,'Pegasoferae',NULL);
INSERT INTO "lores" VALUES ('Caniformia',NULL,'Pegasoferae',NULL);
INSERT INTO "lores" VALUES ('Pholidota',NULL,'Pegasoferae',NULL);
INSERT INTO "lores" VALUES ('Felinae',NULL,'Feliformia',NULL);
INSERT INTO "lores" VALUES ('Viverroidea',NULL,'Feliformia',NULL);
INSERT INTO "lores" VALUES ('Barbourofelidae',NULL,'Feliformia',NULL);
INSERT INTO "lores" VALUES ('Pantherinae',NULL,'Feliformia',NULL);
INSERT INTO "lores" VALUES ('Ceratomorpha',NULL,'Perissodactyla',NULL);
INSERT INTO "lores" VALUES ('Equidae',NULL,'Perissodactyla',NULL);
INSERT INTO "lores" VALUES ('Canidae',NULL,'Caniformia',NULL);
INSERT INTO "lores" VALUES ('Ursidae',NULL,'Caniformia',NULL);
INSERT INTO "lores" VALUES ('Musteloidea',NULL,'Caniformia',NULL);
INSERT INTO "lores" VALUES ('Pinnipedia',NULL,'Caniformia',NULL);
INSERT INTO "lores" VALUES ('Amphicyonidae',NULL,'Caniformia',NULL);
INSERT INTO "lores" VALUES ('Sciuromorpha',NULL,'Rodentia',NULL);
INSERT INTO "lores" VALUES ('Myomorpha',NULL,'Rodentia',NULL);
INSERT INTO "lores" VALUES ('Hystricomorpha',NULL,'Rodentia',NULL);
INSERT INTO "lores" VALUES ('Castorimorpha',NULL,'Rodentia',NULL);
INSERT INTO "lores" VALUES ('Opteromorpha',NULL,'Evarchonta',NULL);
INSERT INTO "lores" VALUES ('Primata',NULL,'Evarchonta',NULL);
INSERT INTO "lores" VALUES ('Tarsiiformes',NULL,'Primata',NULL);
INSERT INTO "lores" VALUES ('Simia',NULL,'Primata',NULL);
INSERT INTO "lores" VALUES ('Irminae',NULL,'Simia',NULL);
INSERT INTO "lores" VALUES ('Jotuni',NULL,'Simia',NULL);
INSERT INTO "lores" VALUES ('Styganthropa',NULL,'Simia',NULL);
INSERT INTO "lores" VALUES ('Shinaria',NULL,'Simia',NULL);
INSERT INTO "lores" VALUES ('Pecora',NULL,'Cetartiodactyla',NULL);
INSERT INTO "lores" VALUES ('Suina',NULL,'Cetartiodactyla',NULL);
INSERT INTO "lores" VALUES ('Cetantodonta',NULL,'Cetartiodactyla',NULL);
INSERT INTO "lores" VALUES ('Tylopoda',NULL,'Cetartiodactyla',NULL);
INSERT INTO "lores" VALUES ('Cervidae',NULL,'Pecora',NULL);
INSERT INTO "lores" VALUES ('Ossiconidae',NULL,'Pecora',NULL);
INSERT INTO "lores" VALUES ('Caprinomorpha',NULL,'Pecora',NULL);
INSERT INTO "lores" VALUES ('Bovinae',NULL,'Pecora',NULL);
INSERT INTO "lores" VALUES ('Synapsida',NULL,'Reptilia',NULL);
INSERT INTO "lores" VALUES ('Anapsida',NULL,'Reptilia',NULL);
INSERT INTO "lores" VALUES ('Lepidosauromorpha',NULL,'Reptilia',NULL);
INSERT INTO "lores" VALUES ('Archosauromorpha',NULL,'Reptilia',NULL);
INSERT INTO "lores" VALUES ('Ichthyosauria',NULL,'Reptilia',NULL);
INSERT INTO "lores" VALUES ('Sauropterygia',NULL,'Lepidosauromorpha',NULL);
INSERT INTO "lores" VALUES ('Serpentes',NULL,'Lepidosauromorpha',NULL);
INSERT INTO "lores" VALUES ('Aigialosauridae',NULL,'Lepidosauromorpha',NULL);
INSERT INTO "lores" VALUES ('Serpentidoidea',NULL,'Lepidosauromorpha',NULL);
INSERT INTO "lores" VALUES ('Lacertilia',NULL,'Lepidosauromorpha',NULL);
INSERT INTO "lores" VALUES ('Ornitischia',NULL,'Archosauromorpha',NULL);
INSERT INTO "lores" VALUES ('Saurischia',NULL,'Archosauromorpha',NULL);
INSERT INTO "lores" VALUES ('Crurotarsi',NULL,'Archosauromorpha',NULL);
INSERT INTO "lores" VALUES ('Pterosauria',NULL,'Archosauromorpha',NULL);
INSERT INTO "lores" VALUES ('Theropoda',NULL,'Saurischia',NULL);
INSERT INTO "lores" VALUES ('Sauropodomorpha',NULL,'Saurischia',NULL);
INSERT INTO "lores" VALUES ('Neognathae',NULL,'Aves',NULL);
INSERT INTO "lores" VALUES ('Paleognathae',NULL,'Aves',NULL);
INSERT INTO "lores" VALUES ('Papyrology',NULL,'Material Lore',NULL);
INSERT INTO "lores" VALUES ('Botany',NULL,'Material Lore',NULL);
INSERT INTO "lores" VALUES ('Metallurgy',NULL,'Material Lore',NULL);
INSERT INTO "lores" VALUES ('Textile Lore',NULL,'Material Lore',NULL);
INSERT INTO "lores" VALUES ('Papyrus Lore',NULL,'Papyrology',NULL);
INSERT INTO "lores" VALUES ('Sarducaan Papyrus Lore',NULL,'Papyrology',NULL);
INSERT INTO "lores" VALUES ('Mammal Skin Lore',NULL,'Animal Materials',NULL);
INSERT INTO "lores" VALUES ('Flesh Lore',NULL,'Animal Materials',NULL);
INSERT INTO "lores" VALUES ('Dental Lore',NULL,'Animal Materials',NULL);
INSERT INTO "lores" VALUES ('Scales Lore',NULL,'Animal Materials',NULL);
INSERT INTO "lores" VALUES ('Skeleton Lore',NULL,'Animal Materials',NULL);
INSERT INTO "lores" VALUES ('Keratin Lore',NULL,'Animal Materials',NULL);
INSERT INTO "lores" VALUES ('Dairy Products Lore',NULL,'Animal Materials',NULL);
INSERT INTO "lores" VALUES ('Fur Lore',NULL,'Mammal Skin Lore',NULL);
INSERT INTO "lores" VALUES ('Leather Lore',NULL,'Mammal Skin Lore',NULL);
INSERT INTO "lores" VALUES ('Guard Fur Lore',NULL,'Fur Lore',NULL);
INSERT INTO "lores" VALUES ('Irnofur Lore',NULL,'Fur Lore',NULL);
INSERT INTO "lores" VALUES ('Ground Fur Lore',NULL,'Fur Lore',NULL);
INSERT INTO "lores" VALUES ('Quality Leather Lore',NULL,'Leather Lore',NULL);
INSERT INTO "lores" VALUES ('Rawhide Lore',NULL,'Leather Lore',NULL);
INSERT INTO "lores" VALUES ('Skin Lore',NULL,'Leather Lore',NULL);
INSERT INTO "lores" VALUES ('Boiled Leather Lore',NULL,'Leather Lore',NULL);
INSERT INTO "lores" VALUES ('Fullgrain Leather Lore',NULL,'Leather Lore',NULL);
INSERT INTO "lores" VALUES ('Brained Leather Lore',NULL,'Leather Lore',NULL);
INSERT INTO "lores" VALUES ('IvoryHide Lore',NULL,'Leather Lore',NULL);
INSERT INTO "lores" VALUES ('Fat and Tallow Lore',NULL,'Flesh Lore',NULL);
INSERT INTO "lores" VALUES ('Glires Meat Lore',NULL,'Flesh Lore',NULL);
INSERT INTO "lores" VALUES ('Pork Lore',NULL,'Flesh Lore',NULL);
INSERT INTO "lores" VALUES ('Primate Meat Lore',NULL,'Flesh Lore',NULL);
INSERT INTO "lores" VALUES ('Herptile Meat Lore',NULL,'Flesh Lore',NULL);
INSERT INTO "lores" VALUES ('Seafood Lore',NULL,'Flesh Lore',NULL);
INSERT INTO "lores" VALUES ('Fowl Meat Lore',NULL,'Flesh Lore',NULL);
INSERT INTO "lores" VALUES ('Carnivora Meat Lore',NULL,'Flesh Lore',NULL);
INSERT INTO "lores" VALUES ('Arthropod Meat Lore',NULL,'Flesh Lore',NULL);
INSERT INTO "lores" VALUES ('Fish Meat Lore',NULL,'Flesh Lore',NULL);
INSERT INTO "lores" VALUES ('Venison Lore',NULL,'Flesh Lore',NULL);
INSERT INTO "lores" VALUES ('Beef Lore',NULL,'Flesh Lore',NULL);
INSERT INTO "lores" VALUES ('Incisium Lore',NULL,'Dental Lore',NULL);
INSERT INTO "lores" VALUES ('Molarium Lore',NULL,'Dental Lore',NULL);
INSERT INTO "lores" VALUES ('Emalj Lore',NULL,'Dental Lore',NULL);
INSERT INTO "lores" VALUES ('Pansar Scales Lore',NULL,'Scales Lore',NULL);
INSERT INTO "lores" VALUES ('Plate Scales Lore',NULL,'Scales Lore',NULL);
INSERT INTO "lores" VALUES ('Keeled Scales Lore',NULL,'Scales Lore',NULL);
INSERT INTO "lores" VALUES ('Horned Scales Lore',NULL,'Scales Lore',NULL);
INSERT INTO "lores" VALUES ('Ganoid Scales Lore',NULL,'Scales Lore',NULL);
INSERT INTO "lores" VALUES ('Leptoid Scales Lore',NULL,'Scales Lore',NULL);
INSERT INTO "lores" VALUES ('Placoid Scales Lore',NULL,'Scales Lore',NULL);
INSERT INTO "lores" VALUES ('Cosmoid Scales Lore',NULL,'Scales Lore',NULL);
INSERT INTO "lores" VALUES ('Endoskeleton Lore',NULL,'Skeleton Lore',NULL);
INSERT INTO "lores" VALUES ('Exoskeleton Lore',NULL,'Skeleton Lore',NULL);
INSERT INTO "lores" VALUES ('Ironbone Lore',NULL,'Endoskeleton Lore',NULL);
INSERT INTO "lores" VALUES ('Bone Tissue Lore',NULL,'Endoskeleton Lore',NULL);
INSERT INTO "lores" VALUES ('Carapace Lore',NULL,'Exoskeleton Lore',NULL);
INSERT INTO "lores" VALUES ('Dense Crepite Lore',NULL,'Exoskeleton Lore',NULL);
INSERT INTO "lores" VALUES ('Crepite Lore',NULL,'Exoskeleton Lore',NULL);
INSERT INTO "lores" VALUES ('Heavy Carapace Lore',NULL,'Carapace Lore',NULL);
INSERT INTO "lores" VALUES ('Reptile Carapace Lore',NULL,'Carapace Lore',NULL);
INSERT INTO "lores" VALUES ('Arthropod Carapace Lore',NULL,'Carapace Lore',NULL);
INSERT INTO "lores" VALUES ('Crustacean Carapace Lore',NULL,'Carapace Lore',NULL);
INSERT INTO "lores" VALUES ('Pansar Carapace Lore',NULL,'Carapace Lore',NULL);
INSERT INTO "lores" VALUES ('Ivory Carapace Lore',NULL,'Carapace Lore',NULL);
INSERT INTO "lores" VALUES ('Horn Lore',NULL,'Keratin Lore',NULL);
INSERT INTO "lores" VALUES ('Compact Horn Lore',NULL,'Keratin Lore',NULL);
INSERT INTO "lores" VALUES ('Great Horn Lore',NULL,'Keratin Lore',NULL);
INSERT INTO "lores" VALUES ('Ampelology',NULL,'Botany',NULL);
INSERT INTO "lores" VALUES ('Pteridology',NULL,'Botany',NULL);
INSERT INTO "lores" VALUES ('Vegetology',NULL,'Botany',NULL);
INSERT INTO "lores" VALUES ('Herbology',NULL,'Botany',NULL);
INSERT INTO "lores" VALUES ('Carpology',NULL,'Botany',NULL);
INSERT INTO "lores" VALUES ('Botanical Oils Lore',NULL,'Botany',NULL);
INSERT INTO "lores" VALUES ('Mycology',NULL,'Botany',NULL);
INSERT INTO "lores" VALUES ('Agrostology',NULL,'Botany',NULL);
INSERT INTO "lores" VALUES ('Dendrology',NULL,'Botany',NULL);
INSERT INTO "lores" VALUES ('Greywood Lore',NULL,'Dendrology',NULL);
INSERT INTO "lores" VALUES ('Whitewood Lore',NULL,'Dendrology',NULL);
INSERT INTO "lores" VALUES ('Spongewood Lore',NULL,'Dendrology',NULL);
INSERT INTO "lores" VALUES ('Dapplewood Lore',NULL,'Dendrology',NULL);
INSERT INTO "lores" VALUES ('Firmwood Lore',NULL,'Dendrology',NULL);
INSERT INTO "lores" VALUES ('Brownwood Lore',NULL,'Dendrology',NULL);
INSERT INTO "lores" VALUES ('Advanced Dendrology',NULL,'Dendrology',NULL);
INSERT INTO "lores" VALUES ('Blackwood Lore',NULL,'Advanced Dendrology',NULL);
INSERT INTO "lores" VALUES ('Ironwood Lore',NULL,'Advanced Dendrology',NULL);
INSERT INTO "lores" VALUES ('Alchemical Mineralogy',NULL,'Petrology',NULL);
INSERT INTO "lores" VALUES ('Mineralogy',NULL,'Petrology',NULL);
INSERT INTO "lores" VALUES ('Igneous Rock',NULL,'Petrology',NULL);
INSERT INTO "lores" VALUES ('Sedimentary Rock',NULL,'Petrology',NULL);
INSERT INTO "lores" VALUES ('Waterstone Lore',NULL,'Alchemical Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Chalk Glance Lore',NULL,'Alchemical Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Sanguinite Lore',NULL,'Alchemical Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Cinnabar Lore',NULL,'Alchemical Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Pyropite Lore',NULL,'Alchemical Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Kyanite Lore',NULL,'Alchemical Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Calamine Lore',NULL,'Alchemical Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Kimurite Lore',NULL,'Alchemical Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Pyrite Lore',NULL,'Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Bleckblende Lore',NULL,'Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Calspar Lore',NULL,'Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Nyx Lore',NULL,'Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Malachite Lore',NULL,'Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Azurite Lore',NULL,'Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Coke Lore',NULL,'Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Jadeite Lore',NULL,'Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Cerulite Lore',NULL,'Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Red Bleckblende Lore',NULL,'Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Blood Ore Lore',NULL,'Mineralogy',NULL);
INSERT INTO "lores" VALUES ('Pyroxene Lore',NULL,'Igneous Rock',NULL);
INSERT INTO "lores" VALUES ('Galbinium Lore',NULL,'Igneous Rock',NULL);
INSERT INTO "lores" VALUES ('Gabore Lore',NULL,'Igneous Rock',NULL);
INSERT INTO "lores" VALUES ('Maalite Lore',NULL,'Igneous Rock',NULL);
INSERT INTO "lores" VALUES ('Amarantum Lore',NULL,'Igneous Rock',NULL);
INSERT INTO "lores" VALUES ('Granum Lore',NULL,'Igneous Rock',NULL);
INSERT INTO "lores" VALUES ('Tephra Lore',NULL,'Igneous Rock',NULL);
INSERT INTO "lores" VALUES ('Magnum Lore',NULL,'Igneous Rock',NULL);
INSERT INTO "lores" VALUES ('Volcanic Ash Lore',NULL,'Igneous Rock',NULL);
INSERT INTO "lores" VALUES ('Glimmer Lore',NULL,'Igneous Rock',NULL);
INSERT INTO "lores" VALUES ('Demorite Lore',NULL,'Igneous Rock',NULL);
INSERT INTO "lores" VALUES ('Titanit Powder Lore',NULL,'Igneous Rock',NULL);
INSERT INTO "lores" VALUES ('Coal Lore',NULL,'Sedimentary Rock',NULL);
INSERT INTO "lores" VALUES ('Flakestone Lore',NULL,'Sedimentary Rock',NULL);
INSERT INTO "lores" VALUES ('Calx Lore',NULL,'Sedimentary Rock',NULL);
INSERT INTO "lores" VALUES ('Saburra Lore',NULL,'Sedimentary Rock',NULL);
INSERT INTO "lores" VALUES ('Precious Metals',NULL,'Metallurgy',NULL);
INSERT INTO "lores" VALUES ('Basic Metals',NULL,'Metallurgy',NULL);
INSERT INTO "lores" VALUES ('Iron-Based Alloys',NULL,'Metallurgy',NULL);
INSERT INTO "lores" VALUES ('Cuprum-Based Alloys',NULL,'Metallurgy',NULL);
INSERT INTO "lores" VALUES ('Silver Lore',NULL,'Precious Metals',NULL);
INSERT INTO "lores" VALUES ('Gold Lore',NULL,'Precious Metals',NULL);
INSERT INTO "lores" VALUES ('Skadite Lore',NULL,'Precious Metals',NULL);
INSERT INTO "lores" VALUES ('Advanced Metals',NULL,'Metallurgy',NULL);
INSERT INTO "lores" VALUES ('Ichor Lore',NULL,'Advanced Metals',NULL);
INSERT INTO "lores" VALUES ('Arcronite Lore',NULL,'Advanced Metals',NULL);
INSERT INTO "lores" VALUES ('Lupium Lore',NULL,'Advanced Metals',NULL);
INSERT INTO "lores" VALUES ('Bleck Lore',NULL,'Basic Metals',NULL);
INSERT INTO "lores" VALUES ('Cuprum Lore',NULL,'Basic Metals',NULL);
INSERT INTO "lores" VALUES ('Almine Lore',NULL,'Basic Metals',NULL);
INSERT INTO "lores" VALUES ('Pig Iron Lore',NULL,'Basic Metals',NULL);
INSERT INTO "lores" VALUES ('Aabam Lore',NULL,'Basic Metals',NULL);
INSERT INTO "lores" VALUES ('Grain Steel Lore',NULL,'Iron-Based Alloys',NULL);
INSERT INTO "lores" VALUES ('Tungsteel Lore',NULL,'Iron-Based Alloys',NULL);
INSERT INTO "lores" VALUES ('Steel Lore',NULL,'Iron-Based Alloys',NULL);
INSERT INTO "lores" VALUES ('Messing Lore',NULL,'Cuprum-Based Alloys',NULL);
INSERT INTO "lores" VALUES ('Tindremic Messing Lore',NULL,'Cuprum-Based Alloys',NULL);
INSERT INTO "lores" VALUES ('Bron Lore',NULL,'Cuprum-Based Alloys',NULL);
INSERT INTO "lores" VALUES ('Electrum Lore',NULL,'Cuprum-Based Alloys',NULL);
INSERT INTO "lores" VALUES ('Ironsilk Lore',NULL,'Textile Lore',NULL);
INSERT INTO "lores" VALUES ('Cotton Lore',NULL,'Textile Lore',NULL);
INSERT INTO "lores" VALUES ('Silk Lore',NULL,'Textile Lore',NULL);
INSERT INTO "lores" VALUES ('Bloodsilk Lore',NULL,'Textile Lore',NULL);
INSERT INTO "lores" VALUES ('Tapii Lore',NULL,'Advanced Dendrology',NULL);
INSERT INTO "lores" VALUES ('Gem Metal Lore',NULL,'Advanced Metals',NULL);

INSERT INTO "lores" VALUES ('Cuprite Lore',NULL,'Alchemy','Alchemical Mineralogy');
INSERT INTO "lores" VALUES ('Arthropod Produce Lore',NULL,'Arthropoda','Animal Materials');
INSERT INTO "lores" VALUES ('Ophionite Lore',NULL,'Dragon Lore','Endoskeleton Lore');
INSERT INTO "lores" VALUES ('Ophiosquami Lore',NULL,'Dragon Lore','Scales Lore');
INSERT INTO "lores" VALUES ('Ophium Lore',NULL,'Dragon Lore','Dental Lore');
INSERT INTO "lores" VALUES ('Ophiodermi Lore',NULL,'Dragon Lore','Scales Lore');
INSERT INTO "lores" VALUES ('Ophiocorni Lore',NULL,'Dragon Lore','Keratin Lore');
INSERT INTO "lores" VALUES ('Wool Lore',NULL,'Fur Lore','Textile Lore');
INSERT INTO "lores" VALUES ('Ironwool Lore',NULL,'Fur Lore','Textile Lore');
INSERT INTO "lores" VALUES ('Stonewood Lore',NULL,'Advanced Dendrology','Petrology');
INSERT INTO "lores" VALUES ('Master Alloys',NULL,'Metallurgy','Advanced Metals');
INSERT INTO "lores" VALUES ('Honey Lore',NULL,'Arthropoda','Arthropod Produce Lore');
INSERT INTO "lores" VALUES ('Beeswax Lore',NULL,'Arthropoda','Arthropod Produce Lore');
INSERT INTO "lores" VALUES ('UNKNOWN',NULL, NULL, NULL);
