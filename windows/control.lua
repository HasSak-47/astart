
local drag_int = 0
local last_value = 0

function Setup ()
    if DragInt == nil then
        print('what')
        return
    end
    print('set up start:')
    drag_int = DragInt.new("test0")
    last_value = DragInt.get(drag_int);
    print('last_value: ', last_value)
    print('set up end')
end

function Loop()
    if last_value ~= DragInt.get(drag_int) then
        last_value = DragInt.get(drag_int)
        print('value changed to ', last_value)
    end

end
