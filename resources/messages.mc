; // Message definitions for windows-event-log

LanguageNames=(English=0x409:MSG00409)

SeverityNames=(
               Informational=0x1:STATUS_SEVERITY_INFORMATIONAL
               Warning=0x2:STATUS_SEVERITY_WARNING
               Error=0x3:STATUS_SEVERITY_ERROR
              )


; // Actual message definitions.

MessageIdTypedef=DWORD

MessageId=0x1
Severity=Error
SymbolicName=ERROR
Language=English
%1
.

MessageId=0x2
Severity=Warning
SymbolicName=WARNING
Language=English
%1
.

MessageId=0x3
Severity=Informational
SymbolicName=INFO
Language=English
%1
.

MessageId=0x4
Severity=Informational
SymbolicName=DEBUG
Language=English
%1
.

MessageId=0x5
Severity=Informational
SymbolicName=TRACE
Language=English
%1
.
