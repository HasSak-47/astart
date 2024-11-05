
local drag_int = 0
local button = 0
local last_value = 0

function Setup ()
    if DragInt == nil or ReloadButton == nil then
        print('what')
        return
    end

    print('set up start:')
    button = ReloadButton.new()
    drag_int = DragInt.new("Test 0")
    last_value = DragInt.get(drag_int);
    print('set up end')
end

function Loop()
    if last_value ~= DragInt.get(drag_int) then
        last_value = DragInt.get(drag_int)
        print('value changed to ', last_value)
    end

end
