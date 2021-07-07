from cuid import cuid
from django.db.models import (
    CharField,
    Model,
    TextChoices,
)
from django.utils.translation import gettext_lazy as _

class Address(Model):
    class State(TextChoices):

        AL = "AL", _("Alabama")
        AK = "AK", _("Alaska")
        AS = "AS", _("American Samoa")
        AZ = "AZ", _("Arizona")
        AR = "AR", _("Arkansas")
        CA = "CA", _("California")
        CO = "CO", _("Colorado")
        CT = "CT", _("Connecticut")
        DE = "DE", _("Delaware")
        DC = "DC", _("District Of Columbia")
        FM = "FM", _("Federated States Of Micronesia")
        FL = "FL", _("Florida")
        GA = "GA", _("Georgia")
        GU = "GU", _("Guam")
        HI = "HI", _("Hawaii")
        ID = "ID", _("Idaho")
        IL = "IL", _("Illinois")
        IN = "IN", _("Indiana")
        IA = "IA", _("Iowa")
        KS = "KS", _("Kansas")
        KY = "KY", _("Kentucky")
        LA = "LA", _("Louisiana")
        ME = "ME", _("Maine")
        MH = "MH", _("Marshall Islands")
        MD = "MD", _("Maryland")
        MA = "MA", _("Massachusetts")
        MI = "MI", _("Michigan")
        MN = "MN", _("Minnesota")
        MS = "MS", _("Mississippi")
        MO = "MO", _("Missouri")
        MT = "MT", _("Montana")
        NE = "NE", _("Nebraska")
        NV = "NV", _("Nevada")
        NH = "NH", _("New Hampshire")
        NJ = "NJ", _("New Jersey")
        NM = "NM", _("New Mexico")
        NY = "NY", _("New York")
        NC = "NC", _("North Carolina")
        ND = "ND", _("North Dakota")
        MP = "MP", _("Northern Mariana Islands")
        OH = "OH", _("Ohio")
        OK = "OK", _("Oklahoma")
        OR = "OR", _("Oregon")
        PW = "PW", _("Palau")
        PA = "PA", _("Pennsylvania")
        PR = "PR", _("Puerto Rico")
        RI = "RI", _("Rhode Island")
        SC = "SC", _("South Carolina")
        SD = "SD", _("South Dakota")
        TN = "TN", _("Tennessee")
        TX = "TX", _("Texas")
        UT = "UT", _("Utah")
        VT = "VT", _("Vermont")
        VI = "VI", _("Virgin Islands")
        VA = "VA", _("Virginia")
        WA = "WA", _("Washington")
        WV = "WV", _("West Virginia")
        WI = "WI", _("Wisconsin")
        WY = "WY", _("Wyoming")

    class Type(TextChoices):
        MAIL = "MAIL", _("Mailing")
        RESIDENTIAL = "RESD", _("Residential")
        BUSINESS = "BUSN", _("Business")

    idempotent_key = CharField(max_length=50, default=cuid)
    address_type = CharField(max_length=4, null=True, blank=True, choices=Type.choices)
    street1 = CharField(max_length=100)
    street2 = CharField(max_length=100, null=True, blank=True)
    state = CharField(max_length=2, choices=State.choices, default=State.TN)
    city = CharField(max_length=50)
    zipcode = CharField(max_length=10)

    def __str__(self):
        return f" {self.street1}  {self.city},  {self.state}  {self.zipcode}"

