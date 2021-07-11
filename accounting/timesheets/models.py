from uuid import uuid4

from django.db.models import (CharField, DateField, DateTimeField, EmailField,
                              Model, UUIDField)
from django.utils.translation import gettext_lazy as _


class Timesheet(Model):
    class Status(TextChoices):
        DRAFT = "DRFT", _("NOT submitted for approval")
        REQUESTED = "REQD", _("Submitted for approval")
        APPROVED = "APPD", _("Approved but the pay run has not been posted")
        DECLINED = "DENY", _("Submitted and declined by the approver")
        COMPLETED = "CMPD", _("Approved and the pay run has been posted")

    # timesheet_id=UUIDField(default=uuid4, null=True, blank=True)
    status = CharField(max_length=4, choices=Status.choices, default=Status.DRAFT)
    start_date = DateField(null=True, blank=True)
    end_date = DateField(null=True, blank=True)
    date_created = DateTimeField(auto_now_add=True, null=True, blank=True)
    date_modified = DateTimeField(auto_now=True, null=True, blank=True)
