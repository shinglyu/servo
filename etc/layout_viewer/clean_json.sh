sed 's/\\\"/\"/g' ../../layout_trace.json | sed 's/\"{/{/g' | sed 's/}\"/}/g' | python -m json.tool

