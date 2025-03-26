CREATE TABLE Users (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    is_admin BOOLEAN DEFAULT FALSE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    last_login TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
 );

CREATE TABLE User_Session (
    id TEXT NOT NULL PRIMARY KEY,
    user_id UUID NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES Users(id)
);

CREATE TYPE Deer_Entry_Status AS ENUM ('Pending', 'Approved', 'Rejected');

CREATE TABLE Cervidae (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    image_url TEXT,
    kill_count BIGINT DEFAULT 0,
    created_by UUID NOT NULL,
    updated_by UUID NOT NULL,
    status Deer_Entry_Status DEFAULT 'Pending' NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (created_by) REFERENCES Users(id),
    FOREIGN KEY (updated_by) REFERENCES Users(id)
);

CREATE TABLE Review (
    user_id UUID NOT NULL,
    cervidae_id UUID NOT NULL,
    danger_level INTEGER NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (user_id, cervidae_id),
    FOREIGN KEY (user_id) REFERENCES Users(id),
    FOREIGN KEY (cervidae_id) REFERENCES Cervidae(id)
);

CREATE TABLE Comment (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    cervidae_id UUID NOT NULL,
    parent_id UUID,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES Users(id),
    FOREIGN KEY (cervidae_id) REFERENCES Cervidae(id),
    FOREIGN KEY (parent_id) REFERENCES Comment(id)
);

CREATE TABLE Crime (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE Crime_Cervidae (
    crime_id UUID NOT NULL,
    cervidae_id UUID NOT NULL,
    PRIMARY KEY (crime_id, cervidae_id),
    FOREIGN KEY (crime_id) REFERENCES Crime(id),
    FOREIGN KEY (cervidae_id) REFERENCES Cervidae(id)
);

/*Populating the tables with some test data: 
Unsafe due to possible UUID collisions, but should be fine for testing*/

INSERT INTO Users VALUES('fabfe0da-9a94-46d3-b380-73cf71246c0b', 'John Doe', 'john.doe@example.com', '$2b$10$MS4CIaBTa1mn9FFW2I.Ve.K5sDQnnNY/FWS7/OrSZpbpdYkJMvRa2', true);
INSERT INTO Users VALUES('fabfe0da-9a94-46d3-b380-73cf71246c0c', 'Jane Doe', 'jane.doe@example.com', '$2b$10$7pXcV2SuKnrQVuaNYTWUgO.F5pA6sn2FAcuNth2TO83qRATZ96BM6', true);
INSERT INTO Users VALUES('fabfe0da-9a94-46d3-b380-73cf71246c0d', 'John Deer', 'john.deer@example.com', '$2b$10$.gXTlKfAytXV8wnwUlH8f.Hb4o7WBqwHf5umOhyy6mqKZoGwPqeke');
INSERT INTO Users VALUES('fabfe0da-9a94-46d3-b380-73cf71246c0e', 'Jane Deer', 'jane.deer@example.com', '$2b$10$FR9Jafg9I9AcMzcz8rv09uNj1K7oAh6/hHKuXwyzQ6rwTdfXGTUsa');
INSERT INTO Users VALUES('fabfe0da-9a94-46d3-b380-73cf71246c0f', 'John Buck', 'john.buck@example.com', '$2b$10$xNVw32Z34DLk.51bHonJcee9Vyj/xjtNeqk8qSEAE3wekzXASlBxy"');
INSERT INTO Users VALUES('fabfe0da-9a94-46d3-b380-73cf71246c0a', 'Jane Buck', 'jane.buck@example.com', '$2b$10$9Ab94NSlRUHgEI88f9matOfBEdnRrCK5M1k/RjjAOnrcR3SXkzL6i');

INSERT INTO Cervidae VALUES('99ad6de2-af4c-4104-86dd-cfb211d249c7', 'Dama dama',
 'Also known as the European fallow deer, these monsters are feared across the Eurasian continent.',
 'https://upload.wikimedia.org/wikipedia/commons/thumb/d/d9/Dama_dama_01.jpg/220px-Dama_dama_01.jpg',
 5923952,
 'fabfe0da-9a94-46d3-b380-73cf71246c0b',
 'fabfe0da-9a94-46d3-b380-73cf71246c0b', 'Approved');
 INSERT INTO Cervidae VALUES('99ad6de2-af4c-4104-86dd-cfb211d249c8', 'Cervus elaphus',
 'Also known as the red deer, their fangs are always red with the blood of their victims.',
 'https://en.wikipedia.org/wiki/Red_deer#/media/File:Cervus_elaphus_Luc_Viatour_6.jpg',
 6231982,
 'fabfe0da-9a94-46d3-b380-73cf71246c0c',
 'fabfe0da-9a94-46d3-b380-73cf71246c0c', 'Approved');
 INSERT INTO Cervidae VALUES('99ad6de2-af4c-4104-86dd-cfb211d249c9', 'Cervus nippon',
 'Also known as the sika deer, northern spotted deer, or the Japanese deer. It is native to much of East Asia and introduced'
 ' to other parts of the world. Its aggressive nature and unquenched bloodlust has earned it the nickname "The Shogun", 
  a famous title bestowed to only the most fearsome of warriors in South West Wales',
 'https://en.wikipedia.org/wiki/Sika_deer#/media/File:Cervus_nippon_002.jpg',
 7213528,
 'fabfe0da-9a94-46d3-b380-73cf71246c0b',
 'fabfe0da-9a94-46d3-b380-73cf71246c0b', 'Approved');
 INSERT INTO Cervidae VALUES('99ad6de2-af4c-4104-86dd-cfb211d249c0', 'Cervus canadensis',
 'Also known as the elk or wapiti, they are the second largest species within the deer family. They utilize their massive size'
 ' to cause Earthquakes across the North American continent. Some have even been known to cause tsunamis. It is said that they do' 
 ' this solely to cause panic and chaos.',
 'https://en.wikipedia.org/wiki/Moose#/media/File:Moose_in_Yellowstone_National_Park_2015.jpg',
 10000000,
 'fabfe0da-9a94-46d3-b380-73cf71246c0c',
 'fabfe0da-9a94-46d3-b380-73cf71246c0c', 'Approved');

INSERT INTO Review VALUES('fabfe0da-9a94-46d3-b380-73cf71246c0d', '99ad6de2-af4c-4104-86dd-cfb211d249c7', 7, 'Honestly kinda mid',
    'No slouch, but they could definitely do better.');
INSERT INTO Review VALUES('fabfe0da-9a94-46d3-b380-73cf71246c0e', '99ad6de2-af4c-4104-86dd-cfb211d249c8', 7, 'Red assholes',
    'One of these guys broke into my garage the other day and stripped my charger for copper. Only found out about it on my security camera.');
INSERT INTO Review VALUES('fabfe0da-9a94-46d3-b380-73cf71246c0f', '99ad6de2-af4c-4104-86dd-cfb211d249c9', 8, 'Hide if you see one of these guys',
    'They can smell blood from miles away. Do not get out until you here the all-clear siren.');
INSERT INTO Review VALUES('fabfe0da-9a94-46d3-b380-73cf71246c0a', '99ad6de2-af4c-4104-86dd-cfb211d249c0', 4, 'Overrated',
    'So called \"exerts\" like to pretend they are some kind of danger. I killed on bare-handed the other day. Not so tough now are they?');
INSERT INTO Review VALUES('fabfe0da-9a94-46d3-b380-73cf71246c0b', '99ad6de2-af4c-4104-86dd-cfb211d249c0', 10, 'Threat to Humanity',
    'They can cause significant seismic activity, yet are nigh unkillable. Humanity is doomed if we do not act soon, and together.');

INSERT INTO Comment VALUES('3881b856-7d08-4a56-a665-35200852572e', 'fabfe0da-9a94-46d3-b380-73cf71246c0f', '99ad6de2-af4c-4104-86dd-cfb211d249c7', null,
    'Demonic in nature.', '2025-02-14 12:00:00', '2025-02-14 12:00:00');
INSERT INTO Comment VALUES('3881b856-7d08-4a56-a665-35200852572f', 'fabfe0da-9a94-46d3-b380-73cf71246c0a', '99ad6de2-af4c-4104-86dd-cfb211d249c7',
    '3881b856-7d08-4a56-a665-35200852572e', 'L + Ratio', '2025-02-14 12:03:32', '2025-02-14 12:03:32');
INSERT INTO Comment VALUES('3881b856-7d08-4a56-a665-352008525720', 'fabfe0da-9a94-46d3-b380-73cf71246c0c', '99ad6de2-af4c-4104-86dd-cfb211d249c0',
    null, 'Fascinating creatures. I would love to have a chat with one.', '2025-02-16 10:08:32', '2025-02-16 10:08:32');
INSERT INTO Comment VALUES('3881b856-7d08-4a56-a665-352008525721', 'fabfe0da-9a94-46d3-b380-73cf71246c0a', '99ad6de2-af4c-4104-86dd-cfb211d249c0',
    '3881b856-7d08-4a56-a665-352008525720', 'How about 8PM Wednesday?', '2025-02-15 18:32:54', '2025-02-15 18:32:54');
INSERT INTO Comment VALUES('3881b856-7d08-4a56-a665-352008525732', 'fabfe0da-9a94-46d3-b380-73cf71246c0e', '99ad6de2-af4c-4104-86dd-cfb211d249c8',
    null, 'Anyone ever seen one of these guys in the wild? I can''t find any video footage of them online.', '2025-02-17 14:01:32', '2025-02-17 14:01:32');
INSERT INTO Comment VALUES('3881b856-7d08-4a56-a665-352008525733', 'fabfe0da-9a94-46d3-b380-73cf71246c0d', '99ad6de2-af4c-4104-86dd-cfb211d249c9',
    null, 'I''m pretty certain the description is incorrect. The title "shogun" originates from native tribes in Southern Mexico, not Wales.', '2025-02-17 06:04:53', '2025-02-17 06:04:53');

  INSERT INTO Crime VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb3d', 'Murder', 'Murder is the unlawful killing of another human without justification or valid excuse committed with the necessary intention as defined by the law in a specific jurisdiction.');
  INSERT INTO Crime VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb3e', 'Assault', 'Assault is the act of causing physical harm or unwanted physical contact to another person, or, in some legal definitions, the threat or attempt to do so.');
  INSERT INTO Crime VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb3f', 'Theft', 'Theft is the act of taking another person''s property or services without that person''s permission or consent with the intent to deprive the rightful owner of it.');
  INSERT INTO Crime VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb40', 'Arson', 'Arson is the act of willfully and deliberately setting fire to or charring property.');
  INSERT INTO Crime VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb41', 'Terrorism', 'Terrorism, in its broadest sense, is the use of violence against non-combatants to achieve political or ideological aims.');
  INSERT INTO Crime VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb42', 'Treason', 'Treason is the crime of attacking a state authority to which one owes allegiance.');
  
  INSERT INTO Crime_Cervidae VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb3d', '99ad6de2-af4c-4104-86dd-cfb211d249c7');
  INSERT INTO Crime_Cervidae VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb3d', '99ad6de2-af4c-4104-86dd-cfb211d249c8');
  INSERT INTO Crime_Cervidae VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb3d', '99ad6de2-af4c-4104-86dd-cfb211d249c9');
  INSERT INTO Crime_Cervidae VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb3d', '99ad6de2-af4c-4104-86dd-cfb211d249c0');
  INSERT INTO Crime_Cervidae VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb3e', '99ad6de2-af4c-4104-86dd-cfb211d249c7');
  INSERT INTO Crime_Cervidae VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb3e', '99ad6de2-af4c-4104-86dd-cfb211d249c8');
  INSERT INTO Crime_Cervidae VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb3e', '99ad6de2-af4c-4104-86dd-cfb211d249c9');
  INSERT INTO Crime_Cervidae VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb3e', '99ad6de2-af4c-4104-86dd-cfb211d249c0');
  INSERT INTO Crime_Cervidae VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb3f', '99ad6de2-af4c-4104-86dd-cfb211d249c7');
  INSERT INTO Crime_Cervidae VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb3f', '99ad6de2-af4c-4104-86dd-cfb211d249c8');
  INSERT INTO Crime_Cervidae VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb40', '99ad6de2-af4c-4104-86dd-cfb211d249c8');
  INSERT INTO Crime_Cervidae VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb40', '99ad6de2-af4c-4104-86dd-cfb211d249c0');
  INSERT INTO Crime_Cervidae VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb41', '99ad6de2-af4c-4104-86dd-cfb211d249c9');
  INSERT INTO Crime_Cervidae VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb41', '99ad6de2-af4c-4104-86dd-cfb211d249c0');
  INSERT INTO Crime_Cervidae VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb42', '99ad6de2-af4c-4104-86dd-cfb211d249c9');
  INSERT INTO Crime_Cervidae VALUES('8c8d502b-f7e1-4ad1-a741-b9dc5bdbdb42', '99ad6de2-af4c-4104-86dd-cfb211d249c8');
 
 