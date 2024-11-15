---
title: Population projections
tags:
  - Demography
  - Population
  - Forecasts
  - Projections
date: 2024-11-15
---


# dataContractSpecification  

    0.9.3

# id  

    urn:datacontract:checkout:orders-latest

# info  


## title  

        Population projections

## version  

        0.1.0

## description  

        Pre-dissemination data ready for ingestion in .Stat .
        SPC validated population projection for the Pacific area. 
        All population estimates, projections or forecasts per country, per age, per sex, per year in the Pacific region. 
        Estimates from 1950 to the current year and projections/forecasts to 2050.
        All derived data (e.g., total population projection, projection for age groups, sex, sub-regions...) are computed by Population projections by SDD - SID - Data Systems

## owner  

        SDD - AI - Demography

## contact  


### name  

            Y. K. (Data Product Owner)

### email  

            y k  at spc.int

# tags  

    Demography
    Population
    Forecasts
    Projections

# servers  


## production  


### type  

            local

### location  

            OneDrive/Path/To/Latest_Population_Projection.csv

### format  

            csv

### delimiter  

            ,

### description  

            Regularly updated csv containing all data.

# terms  


## usage  

        Data is used for the production of demographic, economic, and social indicators.
        Data is suitable for loading on .Stat without further statistical correction.

## confidential  

false

## limitations  

        Not suitable for real-time use cases.

## derived_data  


### sex_total  


#### description  

                Total population projection

#### formula  

                sum(projection) for all sex groups

#### owner  

                SDD - Dissemination

### age_total  


#### description  

                Total population projection

#### formula  

                sum(projection) by five years (0, 1-4, 50-9, 10-14........80-84, 85+) and for all age groups

#### owner  

                SDD - Dissemination

### country_total  


#### description  

                Total population projection

#### formula  

                sum(projection) for all countries

#### owner  

                SDD - Dissemination

### sub_regional_total  


#### description  

                Total population projection

#### formula  

                sum(projection) for all countries in the sub-region

#### owner  

                SDD - Dissemination

# noticePeriod  

    P3M

# models  


## Population_Projection  


### description  

            One population projection per line.

### type  

            table

### fields  


#### year  


##### description  

                    The year the projection refers to.

##### type  

                    numeric

##### required  

true

##### primary  

true

##### example  

2024

#### country  


##### description  

                    Country of projection. Use CL_COM_GEO_PICT codelist.

##### codelist  

                    GEO_PICT

##### type  

                    string

##### required  

true

##### primary  

true

##### example  

                    KI

#### sex  


##### description  

                    The sex the projection refers to. Use CL_COM_SEX codelist.

##### type  

                    string

##### enum  

                    F
                    M

##### required  

true

##### primary  

true

#### age  


##### description  

                    The age the projection refers to. Use CL_COM_AGE codelist.

##### type  

                    int

##### required  

true

##### primary  

true

##### example  

14

#### projection  


##### description  

                    Most accurate and updated population projection per year, country, sex, age. Can be Null if projection is not possible.

##### type  

                    numeric

##### required  

false

#### date_of_projection  


##### description  

                    The date in which projections have been computed and made available

##### type  

                    date

##### required  

true

##### example  

                    2024-10-4

#### data_source  


##### description  

                    Source of numerical value of population projection.

##### type  

                    text

##### required  

true

##### primary  

false

#### obs_status  


##### description  

                    Whether the population projection is Provisional, Estimate, Projection.

##### type  

                    string

##### enum  

                    P
                    E
                    F

##### required  

true

##### primary  

false

#### comment  


##### description  

                    Any additional comment or warning regarding one single population projection.

##### type  

                    text

##### required  

false

##### primary  

false

# examples  


## type  

        csv

## model  

        Population_Projection

## description  

        One population projection per line

## data  

        year,country,sex,age,projection,data_source,obs_status,comment
        1985,KI,F,38,1405,"UN data population projection",E,"Estimated by UN population in 1995"
        2024,NC,_T,42,3205,"SPC correction to UN population projection",P,"Provisional estimated obtained by SPC correcting the 2019 forecast by UN population in 1995"
        2050,FJ,M,18,7204,"UN population projection",F,"Forecast by UN population in 2022"

# servicelevels  


## availability  


### description  

            The file Latest_Population_Projection.csv in OneDrive is constantly available

### percentage  

            99.9%

## retention  


### description  

            Whenever the file Latest_Population_Projection.csv, the previous file is archived as [YEAR_OF_VALIDITY]-Population-Projection.csv and preserved for at least 25 years

### period  

            P25Y

### unlimited  

false

## freshness  


### description  

            The current year estimate and the forecasts for subsequence years are available within the first 3 months of the year.

### threshold  

            3M

### timestampField  

            Population_Projection.Date_of_Projection

## frequency  


### description  

            Data is updated every three years

### type  

            batch

### interval  

            yearly

### cron  

            0 0 1 3 *

## backup  


### description  

            Data is backed up yearly, every 25th of February.

### interval  

            yearly

### cron  

            0 0 25 2 *

# quality  


## type  

        SodaCL

## specification  


### checks for Population_Projections  

            row_count equal to |Year| * |Country| * |Sex| * |Age|
            duplicate_count (Year,Country,Sex,Age) = 0
            missing_percentage (Projection) < 0.1%
            freshness (Date_of_Projection) < 3M
            country %in% CL_COM_GEO_PICT(3.0)
            sex %in% CL_COM_SEX(1.0)
