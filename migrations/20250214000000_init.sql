CREATE TABLE Users (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE Cervidae (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    image_url VARCHAR(255),
    kill_count INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_by UUID NOT NULL,
    updated_by UUID NOT NULL,
    FOREIGN KEY (created_by) REFERENCES Users(id),
    FOREIGN KEY (updated_by) REFERENCES Users(id)
);

CREATE TABLE Review (
    user_id UUID NOT NULL,
    cervidae_id UUID NOT NULL,
    danger_level INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
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
    parent_id UUID,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES Users(id),
    FOREIGN KEY (parent_id) REFERENCES Comment(id)
);

CREATE TABLE Crime (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
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
